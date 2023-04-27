use subxt::client::OnlineClient;

use crate::config::PangolinSubxtConfig;
use crate::error::ClientResult;
use crate::types::DarwiniaAccount;

/// Pangolin client
#[derive(Debug, Clone)]
pub struct PangolinClient {
    /// Endpoint
    pub endpoint: String,
    /// Runtime api
    client: OnlineClient<PangolinSubxtConfig>,
    /// Pangolin Account
    account: DarwiniaAccount,
}

impl PangolinClient {
    /// Create a new darwinia client
    pub fn new(
        endpoint: &str,
        client: OnlineClient<PangolinSubxtConfig>,
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

impl PangolinClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangolinClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<PangolinSubxtConfig> {
        &self.client
    }
}
