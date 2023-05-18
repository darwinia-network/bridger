use subxt::client::OnlineClient;

use crate::config::PangoroSubxtConfig;
use crate::error::ClientResult;
use crate::types::DarwiniaAccount;

/// Pangoro client
#[derive(Debug, Clone)]
pub struct PangoroClient {
    /// Endpoint
    pub endpoint: String,
    /// Runtime api
    client: OnlineClient<PangoroSubxtConfig>,
    /// Pangoro Account
    account: DarwiniaAccount,
}

impl PangoroClient {
    /// Create a new darwinia client
    pub fn new(
        endpoint: &str,
        client: OnlineClient<PangoroSubxtConfig>,
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

impl PangoroClient {
    /// Get darwinia account
    pub fn account(&self) -> &DarwiniaAccount {
        &self.account
    }
}

/// patch rpc api
impl PangoroClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &OnlineClient<PangoroSubxtConfig> {
        &self.client
    }
}
