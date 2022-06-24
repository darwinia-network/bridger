use std::ops::RangeInclusive;

use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::strategy::{RelayReference, RelayStrategy};

use crate::types::LaneId;

/// enforcement decide reference
pub struct EnforcementDecideReference {
    pub lane: LaneId,
    /// nonces
    pub nonces: RangeInclusive<u64>,
    /// message size
    pub message_size: usize,
    /// total weight
    pub total_weight: u64,
}

pub struct EnforcementRelayStrategy<Strategy: RelayStrategy> {
    strategy: Strategy,
}

impl<Strategy: RelayStrategy> EnforcementRelayStrategy<Strategy> {
    pub fn new(strategy: Strategy) -> Self {
        Self { strategy }
    }
}

impl<Strategy: RelayStrategy> EnforcementRelayStrategy<Strategy> {
    pub async fn decide(&mut self, reference: EnforcementDecideReference) -> S2SClientResult<bool> {
        let nonces = &reference.nonces;
        let mut message_nonce = *nonces.start();
        while message_nonce <= *nonces.end() {
            let decide_reference = RelayReference {
                lane: reference.lane,
                nonce: message_nonce,
            };
            let result = self.strategy.decide(decide_reference).await?;
            if !result {
                return Ok(false);
            }
            message_nonce += 1;
        }
        Ok(true)
    }
}
