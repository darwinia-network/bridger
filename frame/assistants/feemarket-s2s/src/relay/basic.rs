use std::ops::Range;

use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::{Chain, ChainBase};
use relay_utils::MaybeConnectionError;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};

/// Basic relay strategy
/// 1. if you are assigned relayer you will relay all order whether or not it times out
/// 2. if you aren't assigned relayer, only participate in the part about time out, earn more rewards
/// 3. if not have any assigned relayers, everyone participates in the relay.
#[derive(Clone)]
pub struct BasicRelayStrategy<A: FeemarketApi> {
    api: A,
    account: <A::Chain as ChainBase>::AccountId,
}

impl<A: FeemarketApi> BasicRelayStrategy<A> {
    pub fn new(api: A, account: <A::Chain as ChainBase>::AccountId) -> Self {
        Self { api, account }
    }
}

impl<A: FeemarketApi> BasicRelayStrategy<A> {
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
            target: "feemarket",
            "[feemarket] [relay] [{}] Determine whether to relay for nonce: {}",
            A::Chain::NAME,
            nonce,
        );
        let order = self.api.order(self.api.lane_id(), *nonce).await?;

        // If the order is not exists.
        // 1. You are too behind.
        // 2. The network question
        // So, you can skip this currently
        // Related: https://github.com/darwinia-network/darwinia-common/blob/90add536ed320ec7e17898e695c65ee9d7ce79b0/frame/fee-market/src/lib.rs?#L177
        if order.is_none() {
            tracing::info!(
                target: "feemarket",
                "[feemarket] [relay] [{}] Not found order by nonce: {}, so decide don't relay this nonce",
                A::Chain::NAME,
                nonce,
            );
            return Ok(false);
        }
        // -----

        let order = order.unwrap();
        let relayers = order.relayers;

        // If not have any assigned relayers, everyone participates in the relay.
        if relayers.is_empty() {
            tracing::info!(
                target: "feemarket",
                "[feemarket] [relay] [{}] Not found any assigned relayers so relay this nonce({}) anyway",
                A::Chain::NAME,
                nonce,
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
                target: "feemarket",
                "[feemarket] [relay] [{}] You are assigned relayer, you must be relay this nonce({})",
                A::Chain::NAME,
                nonce,
            );
            return Ok(true);
        }

        // -----

        // If you aren't assigned relayer, only participate in the part about time out, earn more rewards
        let latest_block_number = self.api.best_finalized_header_number().await?;
        let ranges = relayers
            .iter()
            .map(|item| item.valid_range.clone())
            .collect::<Vec<Range<<A::Chain as ChainBase>::BlockNumber>>>();

        let mut maximum_timeout: <A::Chain as ChainBase>::BlockNumber = Default::default();
        for range in ranges {
            maximum_timeout = std::cmp::max(maximum_timeout, range.end);
        }
        // If this order has timed out, decide to relay
        if latest_block_number > maximum_timeout {
            tracing::info!(
                target: "feemarket",
                "[feemarket] [relay] [{}] You aren't assigned relayer. but this nonce is timeout. so the decide is relay this nonce: {}",
                A::Chain::NAME,
                nonce,
            );
            return Ok(true);
        }
        tracing::info!(
            target: "feemarket",
            "[feemarket] [relay] [{}] You aren't assigned relay. and this nonce({}) is ontime. so don't relay this",
            A::Chain::NAME,
            nonce,
        );
        Ok(false)
    }
}

#[async_trait::async_trait]
impl<A: FeemarketApi> RelayStrategy for BasicRelayStrategy<A> {
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
                    "[feemarket] [relay] [{}] Try decide failed many times ({}). so decide don't relay this nonce({}) at the moment",
                    A::Chain::NAME,
                    times,
                    reference.nonce
                );
                return false;
            }

            let decide = match self.handle(reference).await {
                Ok(v) => v,
                Err(e) => {
                    if let FeemarketError::RelayClient(rce) = &e {
                        if rce.is_connection_error() {
                            if let Err(fe) = self.api.reconnect() {
                                tracing::error!(
                                    target: "feemarket",
                                    "[feemarket] [relay] [{}] Failed reconnect client: {:?}",
                                    A::Chain::NAME,
                                    fe,
                                );
                            }
                        }
                    }
                    tracing::error!(
                        target: "feemarket",
                        "[feemarket] [relay] [{}] Failed to decide relay: {:?}",
                        A::Chain::NAME,
                        e,
                    );
                    continue;
                }
            };
            tracing::info!(
                target: "feemarket",
                "[feemarket] [relay] [{}] About nonce {} decide is {}",
                A::Chain::NAME,
                reference.nonce,
                decide
            );
            return decide;
        }
    }
}
