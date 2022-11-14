use subxt::Client;

use crate::config::DarwiniaParachainSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Darwinia client
#[derive(Clone)]
pub struct DarwiniaParachainClient {
    /// Runtime api
    client: Client<DarwiniaParachainSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl DarwiniaParachainClient {
    /// Create a new darwinia client
    pub fn new(client: Client<DarwiniaParachainSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl DarwiniaParachainClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl DarwiniaParachainClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<DarwiniaParachainSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<DarwiniaParachainSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
