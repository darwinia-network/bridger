use subxt::OnlineClient;

use crate::config::RococoSubxtConfig;
use crate::types::RococoAccount;

/// Rococo client
#[derive(Clone)]
pub struct RococoClient {
    /// Runtime api
    client: OnlineClient<RococoSubxtConfig>,
    /// Darwinia Account
    account: RococoAccount,
}

impl RococoClient {
    /// Create a new Rococo client
    pub fn new(client: OnlineClient<RococoSubxtConfig>, account: RococoAccount) -> Self {
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
    pub fn subxt(&self) -> &OnlineClient<RococoSubxtConfig> {
        &self.client
    }
}
