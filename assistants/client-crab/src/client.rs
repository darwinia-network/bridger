use subxt::client::OnlineClient;

use crate::config::CrabSubxtConfig;
use crate::types::DarwiniaAccount;

/// Crab client
#[derive(Debug, Clone)]
pub struct CrabClient {
    /// Runtime api
    client: OnlineClient<CrabSubxtConfig>,
    /// Crab Account
    account: DarwiniaAccount,
}

impl CrabClient {
    /// Create a new darwinia client
    pub fn new(client: OnlineClient<CrabSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl CrabClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl CrabClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<CrabSubxtConfig> {
        &self.client
    }
}
