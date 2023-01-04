use std::ops::Range;

use bridge_s2s_traits::error::{S2SClientError, S2SClientResult};
use bridge_s2s_traits::strategy::{RelayReference, RelayStrategy};

use feemarket_s2s_traits::api::FeemarketApiRelay;
use feemarket_s2s_traits::types::Chain;
use support_toolkit::logk;

/// Basic relay strategy
/// 1. if you are assigned relayer you will relay all order whether or not it times out
/// 2. if you aren't assigned relayer, only participate in the part about time out, earn more rewards
/// 3. if not have any assigned relayers, everyone participates in the relay.
// #[derive(Clone)]
pub struct BasicRelayStrategy<A: FeemarketApiRelay> {
    api: A,
    account: <A::Chain as Chain>::AccountId,
}

impl<A: FeemarketApiRelay> BasicRelayStrategy<A> {
    pub fn new(api: A, account: <A::Chain as Chain>::AccountId) -> Self {
        Self { api, account }
    }
}

impl<A: FeemarketApiRelay> Clone for BasicRelayStrategy<A> {
    fn clone(&self) -> Self {
        Self {
            api: self.api.clone(),
            account: self.account.clone(),
        }
    }
}

#[async_trait::async_trait]
impl<A: FeemarketApiRelay> RelayStrategy for BasicRelayStrategy<A> {
    async fn decide(&mut self, reference: RelayReference) -> S2SClientResult<bool> {
        let lane = reference.lane;
        let nonce = reference.nonce;
        tracing::trace!(
            target: "feemarket",
            "{} determine whether to relay for nonce: {}",
            logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
            nonce,
        );
        let order = self
            .api
            .order(lane, nonce)
            .await
            .map_err(|e| S2SClientError::Custom(format!("[feemarket]: {:?}", e)))?;

        // If the order is not exists.
        // 1. You are too behind.
        // 2. The network question
        // So, you can skip this currently
        // Related: https://github.com/darwinia-network/darwinia-common/blob/90add536ed320ec7e17898e695c65ee9d7ce79b0/frame/fee-market/src/lib.rs?#L177
        if order.is_none() {
            tracing::info!(
                target: "feemarket",
                "{} not found order by nonce: {}, so decide don't relay this nonce",
                logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
                nonce,
            );
            return Ok(false);
        }
        // -----

        let order = order.unwrap();
        let relayers = order.assigned_relayers;

        // If not have any assigned relayers, everyone participates in the relay.
        if relayers.is_empty() {
            tracing::info!(
                target: "feemarket",
                "{} not found any assigned relayers so relay this nonce({}) anyway",
                logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
                nonce,
            );
            return Ok(true);
        }

        // -----
        let prior_relayer = relayers.iter().find(|item| item.id == self.account);
        let is_assigned_relayer = prior_relayer.is_some();

        let finalized_block_number = self
            .api
            .finalized_header_number()
            .await
            .map_err(|e| S2SClientError::Custom(format!("[feemarket]: {:?}", e)))?;

        // If you are assigned relayer, you must relay this nonce.
        // If you don't do that, the fee market pallet will slash your deposit.
        // Even though it is a timeout, although it will slash your deposit after the timeout is delivered,
        // you can still get relay rewards.
        if is_assigned_relayer {
            let relayer = prior_relayer.unwrap();
            let valid_range = &relayer.valid_range;
            if valid_range.contains(&finalized_block_number) {
                tracing::info!(
                    target: "feemarket",
                    "{} you are assigned relayer and this order is in your slot, you must be relay this nonce({})",
                    logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
                    nonce,
                );
                return Ok(true);
            }
            tracing::info!(
                target: "feemarket",
                "{} you are assigned relayer but this order isn't in your slot, skip this nonce({})",
                logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
                nonce,
            );
            return Ok(false);
        }

        // -----
        // If you aren't assigned relayer, only participate in the part about time out, earn more rewards
        let ranges = relayers
            .iter()
            .map(|item| item.valid_range.clone())
            .collect::<Vec<Range<<A::Chain as Chain>::BlockNumber>>>();

        let mut maximum_timeout: <A::Chain as Chain>::BlockNumber = Default::default();
        for range in ranges {
            maximum_timeout = std::cmp::max(maximum_timeout, range.end);
        }
        // If this order has timed out, decide to relay
        if finalized_block_number > maximum_timeout {
            tracing::info!(
                target: "feemarket",
                "{} you aren't assigned relayer. but this nonce is timeout. so the decide is relay this nonce: {}",
                logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
                nonce,
            );
            return Ok(true);
        }
        tracing::info!(
            target: "feemarket",
            "{} you aren't assigned relay. and this nonce({}) is on-time. so don't relay this",
            logk::prefix_with_relation("feemarket", "relay", A::CHAIN, "::"),
            nonce,
        );
        Ok(false)
    }
}
