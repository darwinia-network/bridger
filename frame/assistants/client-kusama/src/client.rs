use subxt::Client;

use crate::config::KusamaSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{KusamaAccount, NodeRuntimeSignedExtra};

/// Kusama client
#[derive(Clone)]
pub struct KusamaClient {
    /// Runtime api
    client: Client<KusamaSubxtConfig>,
    /// Darwinia Account
    account: KusamaAccount,
}

impl KusamaClient {
    /// Create a new Kusama client
    pub fn new(client: Client<KusamaSubxtConfig>, account: KusamaAccount) -> Self {
        Self { client, account }
    }
}

impl KusamaClient {
    /// Get darwinia account
    pub fn account(&self) -> &KusamaAccount {
        &self.account
    }
}

/// patch rpc api
impl KusamaClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<KusamaSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<KusamaSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
