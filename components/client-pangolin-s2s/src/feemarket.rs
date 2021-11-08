use std::ops::Range;

use common_primitives::AccountId;
use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::Client;
use sp_core::storage::StorageKey;

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

#[async_trait::async_trait]
impl RelayStrategy for PangolinRelayStrategy {
    async fn decide<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> bool {
        let nonce = &reference.nonce;
        let order = self
            .api
            .order(bridge_primitives::PANGORO_PANGOLIN_LANE, *nonce)
            .await
            .map_err(|e| {
                log::error!("Failed to query order: {:?}", e);
            })
            .unwrap_or(None);

        // If the order is not exists. Only one possibility,
        // You are too far behind. so, you can skip this nonce directly
        // Related: https://github.com/darwinia-network/darwinia-common/blob/90add536ed320ec7e17898e695c65ee9d7ce79b0/frame/fee-market/src/lib.rs?#L177
        if order.is_none() {
            return false;
        }
        // -----

        let order = order.unwrap();
        let relayers = order.relayers;

        // If not have any assigned relayers, everyone participates in the relay.
        if relayers.is_empty() {
            return true;
        }

        // -----

        let prior_relayer = relayers.iter().find(|item| item.id == self.account);
        let is_assigned_relayer = prior_relayer.is_some();

        // If you are assigned relayer, you must relay this nonce.
        // If you don't do that, the fee market pallet will slash your deposit.
        // Even though it is a timeout, although it will slash your deposit after the timeout is delivered,
        // you can still get relay rewards.
        if is_assigned_relayer {
            return true;
        }

        // -----

        // If you aren't assigned relayer, only participate in the part about time out, earn more rewards
        let latest_block_number = self
            .api
            .best_finalized_header_number()
            .await
            .map_err(|e| {
                log::error!(
                    "Failed to query latest block, unable to decide whether to participate: {:?}",
                    e
                );
            })
            .unwrap_or(0);
        let ranges = relayers
            .iter()
            .map(|item| item.valid_range.clone())
            .collect::<Vec<Range<common_primitives::BlockNumber>>>();

        let mut maximum_timeout = 0;
        for range in ranges {
            maximum_timeout = std::cmp::max(maximum_timeout, range.end);
        }
        // If this order has timed out, decide to relay
        if latest_block_number > maximum_timeout {
            return true;
        }
        false
    }
}
