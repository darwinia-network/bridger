use pangoro_subxt::api::runtime_types::bsc_primitives::BscHeader;
use pangoro_subxt::api::runtime_types::primitive_types::H160;
use pangoro_subxt::api::RuntimeApi;
use subxt::{Client, TransactionEvents};

use crate::config::PangoroSubxtConfig;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Pangoro client
#[derive(Clone)]
pub struct PangoroClient {
    /// Runtime api
    client: Client<PangoroSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangoroClient {
    /// Create a new Pangoro client
    pub fn new(client: Client<PangoroSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl PangoroClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangoroClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<PangoroSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<PangoroSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }

    /// Get finalized checkpoint of chapel header
    pub async fn finalized_checkpoint(&self) -> color_eyre::Result<BscHeader> {
        Ok(self
            .runtime()
            .storage()
            .bsc()
            .finalized_checkpoint(None)
            .await?)
    }

    /// Get finalized authority set
    pub async fn finalized_authority_set(&self) -> color_eyre::Result<Vec<H160>> {
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
    ) -> color_eyre::Result<TransactionEvents<PangoroSubxtConfig>> {
        Ok(self
            .runtime()
            .tx()
            .bsc()
            .relay_finalized_epoch_header(headers)
            .sign_and_submit_then_watch(self.account().signer())
            .await?
            .wait_for_finalized_success()
            .await?)
    }
}
