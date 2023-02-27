use subxt::client::OnlineClient;

use crate::config::PangoroSubxtConfig;
use crate::types::DarwiniaAccount;

/// Pangoro client
#[derive(Debug, Clone)]
pub struct PangoroClient {
    /// Runtime api
    client: OnlineClient<PangoroSubxtConfig>,
    /// Pangoro Account
    account: DarwiniaAccount,
}

impl PangoroClient {
    /// Create a new darwinia client
    pub fn new(client: OnlineClient<PangoroSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl PangoroClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangoroClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<PangoroSubxtConfig> {
        &self.client
    }
}
