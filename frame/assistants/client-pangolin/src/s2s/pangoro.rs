use crate::client::PangolinClient;
use crate::config::PangolinSubxtConfig;
use subxt::rpc::ChainBlock;

use crate::error::{ClientError, ClientResult};

/// bridge pangoro api
pub struct BridgePangoro<'a> {
    client: &'a PangolinClient,
}

impl<'a> BridgePangoro<'a> {
    /// Create ethereum api instace
    pub fn new(client: &'a PangolinClient) -> Self {
        Self { client }
    }
}

impl<'a> BridgePangoro<'a> {
    // /// Query last relayed header for pangoro in pangolin
    // pub async fn best_finalized(&self) -> ClientResult<ChainBlock<PangolinSubxtConfig>> {
    //     let best_finalized = self
    //         .client
    //         .runtime()
    //         .storage()
    //         .bridge_pangoro_grandpa()
    //         .best_finalized(None)
    //         .await?;
    //     let block = self
    //         .client
    //         .subxt()
    //         .rpc()
    //         .block(Some(best_finalized))
    //         .await?
    //         .ok_or_else(ClientError::WrongBlockHash(best_finalized.to_string()))?;
    //     Ok(block)
    // }
}
