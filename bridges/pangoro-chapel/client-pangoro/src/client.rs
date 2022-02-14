use pangoro_subxt::api::RuntimeApi;
use subxt::Client;

use crate::config::PangoroSubxtConfig;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Pangoro client
#[derive(Clone)]
pub struct PangoroClient {
    /// Runtime api
    client: Client<PangoroSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangoroClient {
    /// Create a new Pangoro client
    pub fn new(client: Client<PangoroSubxtConfig>, account: DarwiniaAccount) -> Self {
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
    pub fn subxt(&self) -> &Client<PangoroSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<PangoroSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
