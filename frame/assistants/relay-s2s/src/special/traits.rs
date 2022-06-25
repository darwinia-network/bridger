use abstract_bridge_s2s::client::S2SClientRelay;
use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::types::bp_runtime::Chain;

#[async_trait::async_trait]
pub trait DifferentClientApi<T: S2SClientRelay> {
    /// query best target finalized at source
    async fn best_target_finalized(
        &self,
        at_block: Option<<T::Chain as Chain>::Hash>,
    ) -> S2SClientResult<<T::Chain as Chain>::Hash>;
}
