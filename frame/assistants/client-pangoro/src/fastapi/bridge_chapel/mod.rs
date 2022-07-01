use subxt::TransactionEvents;

use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::subxt_runtime::api::runtime_types::bsc_primitives::BscHeader;
use crate::subxt_runtime::api::runtime_types::primitive_types::H160;

/// patch rpc api
impl PangoroClient {
    /// Get finalized checkpoint of chapel header
    pub async fn finalized_checkpoint(&self) -> ClientResult<BscHeader> {
        Ok(self
            .runtime()
            .storage()
            .bsc()
            .finalized_checkpoint(None)
            .await?)
    }

    /// Get finalized authority set
    pub async fn finalized_authority_set(&self) -> ClientResult<Vec<H160>> {
        Ok(self
            .runtime()
            .storage()
            .bsc()
            .finalized_authorities(None)
            .await?)
    }

    /// Relay headers to Pangoro
    pub async fn relay_finalized_epoch_header(
        &self,
        headers: Vec<BscHeader>,
    ) -> ClientResult<TransactionEvents<PangoroSubxtConfig>> {
        Ok(self
            .runtime()
            .tx()
            .bsc()
            .relay_finalized_epoch_header(headers)
            .sign_and_submit_then_watch(self.account().signer())
            .await?
            .wait_for_finalized_success()
            .await
            .map_err(|e| ClientError::SubxtRuntime(format!("{:?}", e)))?)
    }
}
