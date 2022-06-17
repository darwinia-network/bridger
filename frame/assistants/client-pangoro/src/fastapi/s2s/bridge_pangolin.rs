use subxt::rpc::ChainBlock;

use abstract_client_s2s::client::S2SClientRelay;

use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::ClientResult;

type BundleJustification =
    crate::types::runtime_types::bp_header_chain::justification::GrandpaJustification<
        crate::fastapi::s2s::generic::BundleHeader,
    >;

#[async_trait::async_trait]
impl S2SClientRelay for PangoroClient {
    type Justification = BundleJustification;
    type ChainBlock = ChainBlock<PangoroSubxtConfig>;

    async fn header(&self, hash: Option<Self::Hash>) -> ClientResult<Option<Self::Header>> {
        match self.subxt().rpc().header(hash).await? {
            Some(v) => {
                let v = codec::Encode::encode(&v);
                Ok(Some(codec::Decode::decode(&mut v.as_slice())?))
            }
            None => Ok(None),
        }
    }

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

    async fn submit_finality_proof(
        &self,
        finality_target: Self::Header,
        justification: Self::Justification,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .tx()
            .bridge_pangolin_grandpa()
            .submit_finality_proof(finality_target, justification)
            .sign_and_submit(self.account().signer())
            .await?)
    }
}
