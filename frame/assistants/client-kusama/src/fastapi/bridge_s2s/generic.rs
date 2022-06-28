use abstract_bridge_s2s::error::{S2SClientError, S2SClientResult};
use abstract_bridge_s2s::{
    client::{S2SClientBase, S2SClientGeneric},
    types::{bp_header_chain, bp_runtime::Chain},
};
use sp_runtime::generic::{Block, SignedBlock};
use subxt::rpc::{Subscription, SubscriptionClientT};
use subxt::{sp_core, sp_runtime};

use support_toolkit::convert::SmartCodecMapper;

use crate::client::KusamaClient;

impl S2SClientBase for KusamaClient {
    const CHAIN: &'static str = "kusama";

    type Chain = bp_kusama::Kusama;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

#[async_trait::async_trait]
impl S2SClientGeneric for KusamaClient {
    type InitializationData = bp_header_chain::InitializationData<<Self::Chain as Chain>::Header>;

    async fn subscribe_grandpa_justifications(
        &self,
    ) -> S2SClientResult<Subscription<sp_core::Bytes>> {
        Ok(self
            .subxt()
            .rpc()
            .client
            .subscribe(
                "grandpa_subscribeJustifications",
                None,
                "grandpa_unsubscribeJustifications",
            )
            .await?)
    }

    async fn header(
        &self,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<<Self::Chain as Chain>::Header>> {
        match self.subxt().rpc().header(hash).await? {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }

    async fn block(
        &self,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<SignedBlock<Block<<Self::Chain as Chain>::Header, Self::Extrinsic>>>>
    {
        Ok(self.subxt().rpc().block(hash).await?)
    }

    async fn read_proof(
        &self,
        storage_keys: Vec<sp_core::storage::StorageKey>,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Vec<Vec<u8>>> {
        let read_proof = self.subxt().rpc().read_proof(storage_keys, hash).await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        Ok(proof)
    }

    async fn prepare_initialization_data(&self) -> S2SClientResult<Self::InitializationData> {
        Err(S2SClientError::Custom(format!(
            "[{}] not needed prepare_initialization_data",
            <Self as S2SClientBase>::CHAIN
        )))
    }
}
