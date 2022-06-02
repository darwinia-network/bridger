use crate::subxt_runtime::api::RuntimeApi;
use subxt::rpc::{Subscription, SubscriptionClientT};
use subxt::Client;

use crate::config::CrabSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::types::{DarwiniaAccount, NodeRuntimeSignedExtra};

/// Crab client
#[derive(Clone)]
pub struct CrabClient {
    /// Runtime api
    client: Client<CrabSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl CrabClient {
    /// Create a new crab client
    pub fn new(client: Client<CrabSubxtConfig>, account: DarwiniaAccount) -> Self {
        Self { client, account }
    }
}

impl CrabClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl CrabClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<CrabSubxtConfig> {
        &self.client
    }

    /// Runtime api
    pub fn runtime(&self) -> RuntimeApi<CrabSubxtConfig, NodeRuntimeSignedExtra> {
        self.client.clone().to_runtime_api()
    }
}

impl CrabClient {
    /// Query spec name
    pub async fn spec_name(&self) -> ClientResult<String> {
        let runtime_version = self.subxt().rpc().runtime_version(None).await?;
        let spec_name = runtime_version
            .other
            .get("specName")
            .ok_or_else(|| ClientError::Other("Failed to query spec name".to_string()))?
            .as_str()
            .ok_or_else(|| {
                ClientError::Other("The spec name not found in runtime version".to_string())
            })?;
        Ok(spec_name.to_string())
    }

    /// query header by block number
    pub async fn header_by_number(
        &self,
        number: u32,
    ) -> ClientResult<Option<<CrabSubxtConfig as subxt::Config>::Header>> {
        match self.subxt().rpc().block_hash(Some(number.into())).await? {
            Some(hash) => Ok(self.subxt().rpc().header(Some(hash)).await?),
            None => Ok(None),
        }
    }

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
