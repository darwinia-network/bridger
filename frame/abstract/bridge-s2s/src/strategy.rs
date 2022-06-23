use crate::client::S2SClientRelay;
use crate::error::S2SClientResult;
use crate::types::bp_messages::LaneId;

/// relay decide
#[async_trait::async_trait]
pub trait RelayStrategy<SC: S2SClientRelay, TC: S2SClientRelay>:
    'static + Clone + Send + Sync
{
    /// decide to relay
    async fn decide(
        &mut self,
        reference: RelayReference<'async_trait, SC, TC>,
    ) -> S2SClientResult<bool>;
}

/// decide reference
pub struct RelayReference<'a, SC: S2SClientRelay, TC: S2SClientRelay> {
    pub client_source: &'a SC,
    pub client_target: &'a TC,
    pub lane: LaneId,
    /// nonces
    pub nonce: u64,
}

#[derive(Clone)]
pub struct AlwaysRelayStrategy;

#[async_trait::async_trait]
impl<SC: S2SClientRelay, TC: S2SClientRelay> RelayStrategy<SC, TC> for AlwaysRelayStrategy {
    async fn decide(
        &mut self,
        _reference: RelayReference<'async_trait, SC, TC>,
    ) -> S2SClientResult<bool> {
        Ok(true)
    }
}
