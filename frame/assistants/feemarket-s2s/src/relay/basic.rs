use std::ops::Range;

use bp_darwinia_core::AccountId;
use bp_darwinia_core::BlockNumber;
use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::{Chain, Client, TransactionSignScheme};

use crate::api::FeemarketApi;
use crate::error::FeemarketResult;

/// Basic relay strategy
/// 1. if you are assigned relayer you will relay all order whether or not it times out
/// 2. if you aren't assigned relayer, only participate in the part about time out, earn more rewards
/// 3. if not have any assigned relayers, everyone participates in the relay.
#[derive(Clone)]
pub struct BasicRelayStrategy<C: Chain, S: TransactionSignScheme<Chain = C>> {
    api: FeemarketApi<C, S>,
    account: AccountId,
}

impl<C: Chain, S: TransactionSignScheme<Chain = C>> BasicRelayStrategy<C, S> {
    pub fn new(client: Client<C>, account: AccountId) -> Self {
        Self {
            api: FeemarketApi::new(client),
            account,
        }
    }
}

impl<C: Chain, S: TransactionSignScheme<Chain = C>> BasicRelayStrategy<C, S> {
    async fn handle<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> FeemarketResult<bool> {
        let nonce = &reference.nonce;
        tracing::trace!(
            target: "client-pangolin",
            "[pangolin] Determine whether to relay for nonce: {}",
            nonce
        );
        let order = self
            .api
            // .order(drml_bridge_primitives::PANGORO_PANGOLIN_LANE, *nonce)
            .order(1234.into(), *nonce)
            .await?;

        // If the order is not exists.
        // 1. You are too behind.
        // 2. The network question
        // So, you can skip this currently
        // Related: https://github.com/darwinia-network/darwinia-common/blob/90add536ed320ec7e17898e695c65ee9d7ce79b0/frame/fee-market/src/lib.rs?#L177
        if order.is_none() {
            tracing::info!(
                target: "client-pangolin",
                "[pangolin] Not found order by nonce: {}, so decide don't relay this nonce",
                nonce
            );
            return Ok(false);
        }
        // -----

        let order = order.unwrap();
        let relayers = order.relayers;

        // If not have any assigned relayers, everyone participates in the relay.
        if relayers.is_empty() {
            tracing::info!(
                target: "client-pangolin",
                "[pangolin] Not found any assigned relayers so relay this nonce({}) anyway",
                nonce
            );
            return Ok(true);
        }

        // -----

        let prior_relayer = relayers.iter().find(|item| item.id == self.account);
        let is_assigned_relayer = prior_relayer.is_some();

        // If you are assigned relayer, you must relay this nonce.
        // If you don't do that, the fee market pallet will slash your deposit.
        // Even though it is a timeout, although it will slash your deposit after the timeout is delivered,
        // you can still get relay rewards.
        if is_assigned_relayer {
            tracing::info!(
                target: "client-pangolin",
                "[pangolin] You are assigned relayer, you must be relay this nonce({})",
                nonce
            );
            return Ok(true);
        }

        // -----

        // If you aren't assigned relayer, only participate in the part about time out, earn more rewards
        let latest_block_number = self.api.best_finalized_header_number().await?;
        let ranges = relayers
            .iter()
            .map(|item| item.valid_range.clone())
            .collect::<Vec<Range<BlockNumber>>>();

        let mut maximum_timeout = 0;
        for range in ranges {
            maximum_timeout = std::cmp::max(maximum_timeout, range.end);
        }
        // If this order has timed out, decide to relay
        if latest_block_number > maximum_timeout {
            tracing::info!(
                target: "client-pangolin",
                "[pangolin] You aren't assigned relayer. but this nonce is timeout. so the decide is relay this nonce: {}",
                nonce
            );
            return Ok(true);
        }
        tracing::info!(
            target: "client-pangolin",
            "[pangolin] You aren't assigned relay. and this nonce({}) is ontime. so don't relay this",
            nonce
        );
        Ok(false)
    }
}

#[async_trait::async_trait]
impl<C: Chain, S: TransactionSignScheme<Chain = C>> RelayStrategy for BasicRelayStrategy<C, S> {
    async fn decide<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &mut self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> bool {
        let mut times = 0;
        loop {
            times += 1;
            if times > 5 {
                tracing::error!(
                    target: "feemarket",
                    "[{}] Try decide failed many times ({}). so decide don't relay this nonce({}) at the moment",
                    C::NAME,
                    times,
                    reference.nonce
                );
                return false;
            }

            let decide = match self.handle(reference).await {
                Ok(v) => v,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(target: "feemarket", "[pangolin] Try reconnect to chain");
                            if let Err(re) = self.api.reconnect().await {
                                tracing::error!(
                                    "[pangolin] Failed to reconnect substrate client: {:?}",
                                    re
                                );
                                continue;
                            }
                        }
                    }

                    tracing::error!(target: "client-pangolin","[pangolin] Failed to decide relay: {:?}", e);
                    continue;
                }
            };
            tracing::info!(
                target: "client-pangolin",
                "[pangolin] About nonce {} decide is {}",
                reference.nonce,
                decide
            );
            return decide;
        }
    }
}
