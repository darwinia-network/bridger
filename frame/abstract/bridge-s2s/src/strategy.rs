use crate::error::S2SClientResult;
use crate::types::bp_messages::LaneId;

/// relay decide
#[async_trait::async_trait]
pub trait RelayStrategy: 'static + Clone + Send + Sync {
    /// decide to relay
    async fn decide(&mut self, reference: RelayReference) -> S2SClientResult<bool>;
}

/// decide reference
pub struct RelayReference {
    pub lane: LaneId,
    /// nonces
    pub nonce: u64,
}

#[derive(Clone)]
pub struct AlwaysRelayStrategy;

#[async_trait::async_trait]
impl RelayStrategy for AlwaysRelayStrategy {
    async fn decide(&mut self, _reference: RelayReference) -> S2SClientResult<bool> {
        Ok(true)
    }
}
