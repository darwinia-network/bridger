use std::{thread, time};

use subxt::OnlineClient;

use crate::client::PolkadotClient;
use crate::config::{ClientConfig, PolkadotSubxtConfig};
use crate::error::ClientResult;
use crate::types::PolkadotAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct PolkadotClientComponent;

impl PolkadotClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<PolkadotClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account = PolkadotAccount::new(config.relayer_private_key)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match OnlineClient::<PolkadotSubxtConfig>::from_url(&endpoint).await {
                Ok(client) => Ok(PolkadotClient::new(client, account.clone())),
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
