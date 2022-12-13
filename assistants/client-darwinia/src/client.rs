use subxt::client::OnlineClient;

use crate::config::DarwiniaSubxtConfig;
use crate::types::DarwiniaAccount;

/// Darwinia client
#[derive(Debug, Clone)]
pub struct DarwiniaClient {
    /// Runtime api
    client: OnlineClient<DarwiniaSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl DarwiniaClient {
    /// Create a new darwinia client
    pub fn new(client: OnlineClient<DarwiniaSubxtConfig>, account: DarwiniaAccount) -> Self {
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
    pub fn subxt(&self) -> &OnlineClient<DarwiniaSubxtConfig> {
        &self.client
    }
}
