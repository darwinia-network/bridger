use bridge_s2s_traits::client::S2SParaBridgeClientRelaychain;
use bridge_s2s_traits::error::S2SClientResult;
use bridge_s2s_traits::types::bp_runtime::Chain;
use bridge_s2s_traits::types::{HeadData, ParaId};
use sp_core::storage::StorageKey;
use subxt::storage::StorageKeyPrefix;
use subxt::StorageEntry;
use support_toolkit::convert::SmartCodecMapper;

use crate::client::RococoClient;
use crate::subxt_runtime::api::paras::storage::Heads;
use crate::types::runtime_types::polkadot_parachain;

#[async_trait::async_trait]
impl S2SParaBridgeClientRelaychain for RococoClient {
    fn gen_parachain_head_storage_key(&self, para_id: u32) -> StorageKey {
        Heads(polkadot_parachain::primitives::Id(para_id))
            .key()
            .final_key(StorageKeyPrefix::new::<Heads>())
    }

    async fn para_head_data(
        &self,
        para_id: ParaId,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<HeadData>> {
        let expected_para_id = SmartCodecMapper::map_to(&para_id)?;
        Ok(self
            .runtime()
            .storage()
            .paras()
            .heads(expected_para_id, hash)
            .await?
            .map(|v| HeadData(v.0)))
    }
}
