use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::error::{S2SClientError, S2SClientResult};
use abstract_bridge_s2s::{
    client::{S2SClientBase, S2SClientGeneric},
    types::bp_header_chain,
};
use finality_grandpa::voter_set::VoterSet;
use sp_finality_grandpa::{AuthorityList, ConsensusLog, ScheduledChange};
use sp_runtime::{ConsensusEngineId, DigestItem};
use subxt::rpc::{ClientT, Subscription, SubscriptionClientT};
use subxt::{sp_core, sp_runtime};

use crate::client::PangolinParachainClient;
use crate::error::{ClientError, ClientResult};
use crate::types::runtime_types::bp_header_chain::InitializationData;

const GRANDPA_ENGINE_ID: ConsensusEngineId = *b"FRNK";

impl S2SClientBase for PangolinParachainClient {
    const CHAIN: &'static str = "pangolin";

    type Config = bp_pangolin_parachain::PangolinParachain;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

#[async_trait::async_trait]
impl S2SClientGeneric for PangolinParachainClient {
    type InitializationData = InitializationData<BundleHeader>;

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

    async fn prepare_initialization_data(&self) -> S2SClientResult<Self::InitializationData> {
        Err(S2SClientError::Custom("Not need".to_string()))
    }

    async fn initialize(
        &self,
        initialization_data: Self::InitializationData,
    ) -> S2SClientResult<<Self::Config as Config>::Hash> {
        Err(S2SClientError::Custom("Not need".to_string()))
    }
}
