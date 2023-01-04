use bridge_s2s_traits::client::S2SClientRelay;
use bridge_s2s_traits::error::S2SClientResult;
use bridge_s2s_traits::types::bp_runtime::Chain;
#[cfg(feature = "bridge-parachain")]
use bridge_s2s_traits::{client::S2SParaBridgeClientSolochain, types::ParaId};

#[cfg(feature = "bridge-parachain")]
use support_toolkit::convert::SmartCodecMapper;

use crate::special::traits::DifferentClientApi;

#[cfg(feature = "bridge-parachain")]
pub struct ParachainSpecialClientApi<T: S2SParaBridgeClientSolochain> {
    pub para_id: u32,
    pub client: T,
}

#[cfg(feature = "bridge-parachain")]
#[async_trait::async_trait]
impl<T: S2SParaBridgeClientSolochain> DifferentClientApi<T> for ParachainSpecialClientApi<T> {
    async fn best_target_finalized(
        &self,
        at_block: Option<<T::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<(<T::Chain as Chain>::BlockNumber, <T::Chain as Chain>::Hash)>>
    {
        match self
            .client
            .best_para_heads(ParaId(self.para_id), at_block)
            .await?
        {
            Some(v) => {
                let best_head_hash = v.best_head_hash;
                Ok(Some((
                    SmartCodecMapper::map_to(&best_head_hash.at_relay_block_number)?,
                    SmartCodecMapper::map_to(&best_head_hash.head_hash)?,
                )))
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
    ) -> S2SClientResult<Option<(<T::Chain as Chain>::BlockNumber, <T::Chain as Chain>::Hash)>>
    {
        self.client.best_target_finalized(at_block).await
    }
}
