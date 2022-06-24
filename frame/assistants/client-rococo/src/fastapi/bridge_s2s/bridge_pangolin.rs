use abstract_bridge_s2s::client::S2SParaBridgeClientRelaychain;
use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::types::bp_runtime::Chain;
use abstract_bridge_s2s::types::{HeadData, ParaId};
use sp_core::storage::StorageKey;

use crate::client::RococoClient;

#[async_trait::asynctrait]
impl S2SParaBridgeClientRelaychain for RococoClient {
    fn gen_parachain_head_storage_key(&self, para_id: u32) -> StorageKey {
        todo!()
    }

    async fn para_head_data(
        &self,
        para_id: ParaId,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Option<HeadData>> {
        todo!()
    }
}
