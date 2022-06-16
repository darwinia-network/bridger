use subxt::rpc::ChainBlock;

use abstract_client_s2s::client::S2SClientRelay;

use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::ClientResult;

#[async_trait::async_trait]
impl S2SClientRelay for PangoroClient {
    type ChainBlock = ChainBlock<PangoroSubxtConfig>;

    async fn block(
        &self,
        hash: Option<Self::Hash>,
    ) -> Result<Option<Self::ChainBlock>, Self::Error> {
        Ok(self.subxt().rpc().block(hash).await?)
    }

    async fn best_target_finalized(
        &self,
        at_block: Option<Self::Hash>,
    ) -> ClientResult<subxt::sp_core::H256> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_grandpa()
            .best_finalized(at_block)
            .await?)
    }
}
