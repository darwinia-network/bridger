use abstract_client_s2s::client::S2SClientRelay;
use subxt::rpc::ChainBlock;

use crate::client::PangolinClient;
use crate::config::PangolinSubxtConfig;
use crate::error::ClientResult;

#[async_trait::async_trait]
impl S2SClientRelay for PangolinClient {
    type ChainBlock = ChainBlock<PangolinSubxtConfig>;

    async fn header(&self, hash: Option<Self::Hash>) -> ClientResult<Option<Self::Header>> {
        match self.subxt().rpc().header(hash).await? {
            Some(v) => {
                let v = codec::Encode::encode(&v);
                Ok(Some(codec::Decode::decode(&mut v.as_slice())?))
            }
            None => Ok(None),
        }
    }

    async fn block(&self, hash: Option<Self::Hash>) -> ClientResult<Option<Self::ChainBlock>> {
        Ok(self.subxt().rpc().block(hash).await?)
    }

    async fn best_target_finalized(
        &self,
        at_block: Option<Self::Hash>,
    ) -> ClientResult<subxt::sp_core::H256> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangoro_grandpa()
            .best_finalized(at_block)
            .await?)
    }

    async fn submit_finality_proof(
        &self,
        finality_target: Self::Header,
        justification: Self::Justification,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .tx()
            .bridge_pangoro_grandpa()
            .submit_finality_proof(finality_target, justification)
            .sign_and_submit(self.account().signer())
            .await?)
    }
}
