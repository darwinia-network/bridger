use subxt::client::OnlineClient;

use crate::config::PangolinSubxtConfig;
use crate::types::DarwiniaAccount;

/// Pangolin client
#[derive(Debug, Clone)]
pub struct PangolinClient {
    /// Runtime api
    client: OnlineClient<PangolinSubxtConfig>,
    /// Pangolin Account
    account: DarwiniaAccount,
}

impl PangolinClient {
    /// Create a new darwinia client
    pub fn new(client: OnlineClient<PangolinSubxtConfig>, account: DarwiniaAccount) -> Self {
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
    pub fn subxt(&self) -> &OnlineClient<PangolinSubxtConfig> {
        &self.client
    }
}
