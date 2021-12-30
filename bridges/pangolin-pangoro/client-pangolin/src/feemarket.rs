use std::ops::Range;

use drml_common_primitives::AccountId;
use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::Client;
use relay_utils::MaybeConnectionError;

use crate::api::PangolinApi;
use crate::PangolinChain;

#[derive(Clone)]
pub struct PangolinRelayStrategy {
    api: PangolinApi,
    account: AccountId,
}

impl PangolinRelayStrategy {
    pub fn new(client: Client<PangolinChain>, account: AccountId) -> Self {
        Self {
            api: PangolinApi::new(client),
            account,
        }
    }
}

impl PangolinRelayStrategy {
    async fn handle<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> color_eyre::Result<bool> {
        let nonce = &reference.nonce;
        tracing::trace!("[pangolin] Determine whether to relay for nonce: {}", nonce);
        let order = self
            .api
            .order(drml_bridge_primitives::PANGORO_PANGOLIN_LANE, *nonce)
            .await?;

        // If the order is not exists.
        // 1. You are too behind.
        // 2. The network question
        // So, you can skip this currently
        // Related: https://github.com/darwinia-network/darwinia-common/blob/90add536ed320ec7e17898e695c65ee9d7ce79b0/frame/fee-market/src/lib.rs?#L177
        if order.is_none() {
            tracing::info!(
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
            .collect::<Vec<Range<drml_common_primitives::BlockNumber>>>();

        let mut maximum_timeout = 0;
        for range in ranges {
            maximum_timeout = std::cmp::max(maximum_timeout, range.end);
        }
        // If this order has timed out, decide to relay
        if latest_block_number > maximum_timeout {
            tracing::info!(
                "[pangolin] You aren't assigned relayer. but this nonce is timeout. so the decide is relay this nonce: {}",
                nonce
            );
            return Ok(true);
        }
        tracing::info!(
            "[pangolin] You aren't assigned relay. and this nonce({}) is ontime. so don't relay this",
            nonce
        );
        Ok(false)
    }
}

#[async_trait::async_trait]
impl RelayStrategy for PangolinRelayStrategy {
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
                    "[pangolin] Try decide failed many times ({}). so decide don't relay this nonce({}) at the moment",
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
                            tracing::debug!("[pangolin] Try reconnect to chain");
                            if let Err(re) = self.api.reconnect().await {
                                tracing::error!(
                                    "[pangolin] Failed to reconnect substrate client: {:?}",
                                    re
                                );
                                continue;
                            }
                        }
                    }

                    tracing::error!("[pangolin] Failed to decide relay: {:?}", e);
                    continue;
                }
            };
            tracing::info!(
                "[pangolin] About nonce {} decide is {}",
                reference.nonce,
                decide
            );
            return decide;
        }
    }
}
