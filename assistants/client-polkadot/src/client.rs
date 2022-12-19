use subxt::OnlineClient;

use crate::config::PolkadotSubxtConfig;
use crate::types::PolkadotAccount;

/// Polkadot client
#[derive(Clone)]
pub struct PolkadotClient {
    /// Runtime api
    client: OnlineClient<PolkadotSubxtConfig>,
    /// Darwinia Account
    account: PolkadotAccount,
}

impl PolkadotClient {
    /// Create a new Polkadot client
    pub fn new(client: OnlineClient<PolkadotSubxtConfig>, account: PolkadotAccount) -> Self {
        Self { client, account }
    }
}

impl PolkadotClient {
    /// Get darwinia account
    pub fn account(&self) -> &PolkadotAccount {
        &self.account
    }
}

/// patch rpc api
impl PolkadotClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<PolkadotSubxtConfig> {
        &self.client
    }
}
