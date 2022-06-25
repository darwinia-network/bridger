use abstract_bridge_s2s::client::{S2SClientRelay, S2SParaBridgeClientSolochain};
use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::types::bp_runtime::Chain;
use abstract_bridge_s2s::types::ParaId;
use support_toolkit::convert::SmartCodecMapper;

use crate::special::traits::DifferentClientApi;

pub struct ParachainSpecialClientApi<T: S2SParaBridgeClientSolochain> {
    pub para_id: u32,
    pub client: T,
}

#[async_trait::async_trait]
impl<T: S2SParaBridgeClientSolochain> DifferentClientApi<T> for ParachainSpecialClientApi<T> {
    async fn best_target_finalized(
        &self,
        at_block: Option<<T::Chain as Chain>::Hash>,
    ) -> S2SClientResult<<T::Chain as Chain>::Hash> {
        match self
            .client
            .best_para_heads(ParaId(self.para_id), at_block)
            .await?
        {
            Some(v) => {
                let hash = sp_core::H256(v.head_hash.0);
                Ok(SmartCodecMapper::map_to(&hash)?)
            }
            None => Ok(Default::default()),
        }
    }
}


pub struct SolochainSpecialClientApi<T: S2SClientRelay> {
    pub client: T,
}

#[async_trait::async_trait]
impl<T: S2SClientRelay> DifferentClientApi<T> for SolochainSpecialClientApi<T> {
    async fn best_target_finalized(
        &self,
        at_block: Option<<T::Chain as Chain>::Hash>,
    ) -> S2SClientResult<<T::Chain as Chain>::Hash> {
        self.client.best_target_finalized(at_block).await
    }
}
