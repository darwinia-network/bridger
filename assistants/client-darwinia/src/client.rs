use subxt::client::OnlineClient;

use crate::config::DarwiniaSubxtConfig;
use crate::error::ClientResult;
use crate::types::DarwiniaAccount;

/// Darwinia client
#[derive(Debug, Clone)]
pub struct DarwiniaClient {
    /// Endpoint
    pub endpoint: String,
    /// Runtime api
    client: OnlineClient<DarwiniaSubxtConfig>,
    /// Darwinia Account
    account: DarwiniaAccount,
}

impl DarwiniaClient {
    /// Create a new darwinia client
    pub fn new(
        endpoint: &str,
        client: OnlineClient<DarwiniaSubxtConfig>,
        account: DarwiniaAccount,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            client,
            account,
        }
    }

    pub async fn reconnect_client(&mut self) -> ClientResult<()> {
        self.client = OnlineClient::from_url(&self.endpoint).await?;
        Ok(())
    }
}

impl DarwiniaClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl DarwiniaClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<DarwiniaSubxtConfig> {
        &self.client
    }
}
