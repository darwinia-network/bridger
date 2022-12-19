use subxt::OnlineClient;

use crate::config::KusamaSubxtConfig;
use crate::types::KusamaAccount;

/// Kusama client
#[derive(Clone)]
pub struct KusamaClient {
    /// Runtime api
    client: OnlineClient<KusamaSubxtConfig>,
    /// Darwinia Account
    account: KusamaAccount,
}

impl KusamaClient {
    /// Create a new Kusama client
    pub fn new(client: OnlineClient<KusamaSubxtConfig>, account: KusamaAccount) -> Self {
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
    pub fn subxt(&self) -> &OnlineClient<KusamaSubxtConfig> {
        &self.client
    }
}
