use subxt::Client;

use crate::config::PolkadotSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{PolkadotAccount, NodeRuntimeSignedExtra};

/// Polkadot client
#[derive(Clone)]
pub struct PolkadotClient {
    /// Runtime api
    client: Client<PolkadotSubxtConfig>,
    /// Darwinia Account
    account: PolkadotAccount,
}

impl PolkadotClient {
    /// Create a new Polkadot client
    pub fn new(client: Client<PolkadotSubxtConfig>, account: PolkadotAccount) -> Self {
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
    pub fn subxt(&self) -> &Client<PolkadotSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<PolkadotSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
