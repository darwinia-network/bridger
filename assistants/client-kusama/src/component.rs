use std::{thread, time};

use subxt::OnlineClient;

use crate::client::KusamaClient;
use crate::config::{ClientConfig, KusamaSubxtConfig};
use crate::error::ClientResult;
use crate::types::KusamaAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct KusamaClientComponent;

impl KusamaClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<KusamaClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account = KusamaAccount::new(config.relayer_private_key)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match OnlineClient::<KusamaSubxtConfig>::from_url(&endpoint).await {
                Ok(client) => Ok(KusamaClient::new(client, account.clone())),
                Err(err) => {
                    if attempts < MAX_ATTEMPTS {
                        attempts += 1;
                        wait_secs *= 2; // backoff
                        continue;
                    }
                    Err(err.into())
                }
            };
        }
    }
}
