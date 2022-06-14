use subxt::Client;

use crate::config::PangolinSubxtConfig;
#[cfg(feature = "ethlike-v1")]
use crate::fastapi::ethereum::EthereumApi;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Pangolin client
#[derive(Clone)]
pub struct PangolinClient {
    /// Runtime api
    client: Client<PangolinSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangolinClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl PangolinClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangolinClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<PangolinSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<PangolinSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }

    /// Ethereum api
    #[cfg(feature = "ethlike-v1")]
    pub fn ethereum(&self) -> EthereumApi {
        EthereumApi::new(self)
    }
}
