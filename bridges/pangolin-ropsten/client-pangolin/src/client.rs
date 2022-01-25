use subxt::Client;

use crate::codegen::api::RuntimeApi;
use crate::config::PangolinSubxtConfig;
use crate::ethereum::{EthereumApi, FromEthereumApi, ToEthereumApi};
use crate::types::DarwiniaAccount;

/// Pangolin client
#[derive(Clone)]
pub struct PangolinClient {
    /// Runtime api
    runtime: RuntimeApi<PangolinSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangolinClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self {
            runtime: RuntimeApi::from(client),
            account,
        }
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
        &self.runtime.client
    }
    /// Runtime api
    pub fn runtime(&self) -> &RuntimeApi<PangolinSubxtConfig> {
        &self.runtime
    }

    /// Ethereum api
    pub fn ethereum(&self) -> EthereumApi {
        EthereumApi::new(&self)
    }
}
