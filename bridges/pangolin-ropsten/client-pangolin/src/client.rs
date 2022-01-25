use subxt::Client;

use crate::codegen::api::RuntimeApi;
use crate::config::PangolinSubxtConfig;
use crate::ethereum::EthereumApi;
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
    pub fn ethereum(&self) -> EthereumApi {
        EthereumApi::new(&self)
    }
}
