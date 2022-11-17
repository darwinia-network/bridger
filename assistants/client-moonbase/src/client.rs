use subxt::Client;

use crate::config::MoonbaseSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{MoonbaseAccount, NodeRuntimeSignedExtra};

/// Moonbase client
#[derive(Clone)]
pub struct MoonbaseClient {
    /// Runtime api
    client: Client<MoonbaseSubxtConfig>,
    /// Darwinia Account
    account: MoonbaseAccount,
}

impl MoonbaseClient {
    /// Create a new Moonbase client
    pub fn new(client: Client<MoonbaseSubxtConfig>, account: MoonbaseAccount) -> Self {
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
    pub fn subxt(&self) -> &Client<MoonbaseSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<MoonbaseSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
