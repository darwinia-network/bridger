use crate::error::ClientResult;
use crate::subxt_runtime::api::RuntimeApi;
use subxt::rpc::{Subscription, SubscriptionClientT};
use subxt::Client;

use crate::config::KusamaSubxtConfig;
use crate::types::{KusamaAccount, NodeRuntimeSignedExtra};

/// Kusama client
#[derive(Clone)]
pub struct KusamaClient {
    /// Runtime api
    client: Client<KusamaSubxtConfig>,
    /// Darwinia Account
    account: KusamaAccount,
}

impl KusamaClient {
    /// Create a new Kusama client
    pub fn new(client: Client<KusamaSubxtConfig>, account: KusamaAccount) -> Self {
        Self { client, account }
    }
}

impl KusamaClient {
    /// Get darwinia account
    pub fn account(&self) -> &KusamaAccount {
        &self.account
    }
}

/// patch rpc api
impl KusamaClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<KusamaSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<KusamaSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}

impl KusamaClient {
    pub async fn subscribe_grandpa_justifications(
        &self,
    ) -> ClientResult<Subscription<sp_core::Bytes>> {
        let sub = self
            .client
            .rpc()
            .client
            .subscribe(
                "grandpa_subscribeJustifications",
                None,
                "grandpa_unsubscribeJustifications",
            )
            .await?;
        Ok(sub)
    }
}
