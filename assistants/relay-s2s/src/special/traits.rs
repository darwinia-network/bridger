use bridge_s2s_traits::client::S2SClientRelay;
use bridge_s2s_traits::error::S2SClientResult;
use bridge_s2s_traits::types::bp_runtime::Chain;

#[async_trait::async_trait]
pub trait DifferentClientApi<T: S2SClientRelay> {
    /// query best target finalized at source
    async fn best_target_finalized(
        &self,
        at_block: Option<<T::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<(<T::Chain as Chain>::BlockNumber, <T::Chain as Chain>::Hash)>>;
}
