use std::{thread, time};

use subxt::OnlineClient;

use crate::client::MoonbaseClient;
use crate::config::{ClientConfig, MoonbaseSubxtConfig};
use crate::error::ClientResult;
use crate::types::MoonbaseAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct MoonbaseClientComponent;

impl MoonbaseClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<MoonbaseClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account = MoonbaseAccount::new(config.relayer_private_key)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match OnlineClient::<MoonbaseSubxtConfig>::from_url(&endpoint).await {
                Ok(client) => Ok(MoonbaseClient::new(client, account.clone())),
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
