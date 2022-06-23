use std::marker::PhantomData;
use std::ops::RangeInclusive;

use abstract_bridge_s2s::client::S2SClientRelay;
use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::strategy::{RelayReference, RelayStrategy};

use crate::helpers;

/// enforcement decide reference
pub struct EnforcementDecideReference<'a, SC: S2SClientRelay, TC: S2SClientRelay> {
    pub client_source: &'a SC,
    pub client_target: &'a TC,
    /// nonces
    pub nonces: RangeInclusive<u64>,
    /// message size
    pub message_size: usize,
    /// total weight
    pub total_weight: u64,
}

pub struct EnforcementRelayStrategy<
    SC: S2SClientRelay,
    TC: S2SClientRelay,
    Strategy: RelayStrategy<SC, TC>,
> {
    strategy: Strategy,
    _mark0: PhantomData<SC>,
    _mark1: PhantomData<TC>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay, Strategy: RelayStrategy<SC, TC>>
    EnforcementRelayStrategy<SC, TC, Strategy>
{
    pub fn new(strategy: Strategy) -> Self {
        Self {
            strategy,
            _mark0: Default::default(),
            _mark1: Default::default(),
        }
    }
}

impl<'a, SC: S2SClientRelay, TC: S2SClientRelay, Strategy: RelayStrategy<SC, TC>>
    EnforcementRelayStrategy<SC, TC, Strategy>
{
    pub async fn decide(
        &mut self,
        reference: EnforcementDecideReference<'a, SC, TC>,
    ) -> S2SClientResult<bool> {
        let nonces = &reference.nonces;
        let mut message_nonce = *nonces.start();
        while message_nonce <= *nonces.end() {
            let decide_reference = RelayReference {
                client_source: reference.client_source,
                client_target: reference.client_target,
                nonce: message_nonce,
            };
            let result = self.strategy.decide(decide_reference).await?;
            if !result {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} the nonce({}) relay strategy decide is false",
                    helpers::log_prefix("strategy", SC::CHAIN, TC::CHAIN),
                    message_nonce,
                );
                return Ok(false);
            }
            message_nonce += 1;
        }
        Ok(true)
    }
}
