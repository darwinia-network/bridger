use bridge_s2s_traits::client::S2SParaBridgeClientRelaychain;
use bridge_s2s_traits::error::S2SClientResult;
use bridge_s2s_traits::types::bp_runtime::Chain;
use bridge_s2s_traits::types::{HeadData, ParaId};

use crate::client::RococoClient;

#[async_trait::async_trait]
impl S2SParaBridgeClientRelaychain for RococoClient {
    fn gen_parachain_head_storage_key(&self, para_id: u32) -> Vec<u8> {
        let address = crate::subxt_runtime::api::storage().paras().heads(
            &crate::subxt_runtime::api::runtime_types::polkadot_parachain::primitives::Id(para_id),
        );
        address.to_bytes()
    }

    async fn para_head_data(
        &self,
        para_id: ParaId,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<HeadData>> {
        let address = crate::subxt_runtime::api::storage().paras().heads(
            &crate::subxt_runtime::api::runtime_types::polkadot_parachain::primitives::Id(para_id.0),
        );
        Ok(self
            .subxt()
            .storage()
            .fetch(&address, hash)
            .await?
            .map(|v| HeadData(v.0)))
    }
}
