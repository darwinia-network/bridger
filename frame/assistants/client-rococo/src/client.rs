use crate::error::ClientResult;
use crate::subxt_runtime::api::RuntimeApi;
use subxt::Client;
use subxt::rpc::{Subscription, SubscriptionClientT};

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

impl RococoClient {

    pub async fn subscribe_grandpa_justifications(&self) -> ClientResult<Subscription<sp_core::Bytes>> {
        let sub = self.client.rpc().client.subscribe(
            "grandpa_subscribeJustifications",
            None,
            "grandpa_unsubscribeJustifications",
        ).await.unwrap();
        Ok(sub)
    }

}