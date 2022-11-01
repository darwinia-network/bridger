use subxt::Client;

use crate::config::PangolinParachainSubxtConfig;
use crate::subxt_runtime::api::RuntimeApi;
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Pangolin client
#[derive(Clone)]
pub struct PangolinParachainClient {
    /// Runtime api
    client: Client<PangolinParachainSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl PangolinParachainClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinParachainSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl PangolinParachainClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangolinParachainClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<PangolinParachainSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<PangolinParachainSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
