use subxt::OnlineClient;

use crate::config::MoonbaseSubxtConfig;
use crate::types::MoonbaseAccount;

/// Moonbase client
#[derive(Clone)]
pub struct MoonbaseClient {
    /// Runtime api
    client: OnlineClient<MoonbaseSubxtConfig>,
    /// Darwinia Account
    account: MoonbaseAccount,
}

impl MoonbaseClient {
    /// Create a new Moonbase client
    pub fn new(client: OnlineClient<MoonbaseSubxtConfig>, account: MoonbaseAccount) -> Self {
        Self { client, account }
    }
}

impl MoonbaseClient {
    /// Get darwinia account
    pub fn account(&self) -> &MoonbaseAccount {
        &self.account
    }
}

/// patch rpc api
impl MoonbaseClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<MoonbaseSubxtConfig> {
        &self.client
    }
}
