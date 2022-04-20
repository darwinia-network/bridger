use rococo_subxt::api::RuntimeApi;
use subxt::Client;

use crate::config::RococoSubxtConfig;
use crate::types::{NodeRuntimeSignedExtra, RococoAccount};

/// Rococo client
#[derive(Clone)]
pub struct RococoClient {
    /// Runtime api
    client: Client<RococoSubxtConfig>,
    /// Darwinia Account
    account: RococoAccount,
}

impl RococoClient {
    /// Create a new rococo client
    pub fn new(client: Client<RococoSubxtConfig>, account: RococoAccount) -> Self {
        Self { client, account }
    }
}

impl RococoClient {
    /// Get darwinia account
    pub fn account(&self) -> &RococoAccount {
        &self.account
    }
}

/// patch rpc api
impl RococoClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<RococoSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<RococoSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}
