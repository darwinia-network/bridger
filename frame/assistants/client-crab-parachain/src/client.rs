use subxt::Client;

use crate::config::CrabParachainSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Crab client
#[derive(Clone)]
pub struct CrabParachainClient {
    /// Runtime api
    client: Client<CrabParachainSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl CrabParachainClient {
    /// Create a new crab client
    pub fn new(client: Client<CrabParachainSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl CrabParachainClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl CrabParachainClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<CrabParachainSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<CrabParachainSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
