use subxt::Client;

use crate::config::DarwiniaSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Darwinia client
#[derive(Clone)]
pub struct DarwiniaClient {
    /// Runtime api
    client: Client<DarwiniaSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl DarwiniaClient {
    /// Create a new darwinia client
    pub fn new(client: Client<DarwiniaSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl DarwiniaClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl DarwiniaClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<DarwiniaSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<DarwiniaSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
