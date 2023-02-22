use std::{thread, time};

use subxt::OnlineClient;

use crate::client::PangolinClient;
use crate::config::ClientConfig;
use crate::config::PangolinSubxtConfig;
use crate::error::ClientResult;
use crate::types::DarwiniaAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct PangolinClientComponent;

impl PangolinClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<PangolinClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account =
            DarwiniaAccount::new(config.relayer_private_key, config.relayer_real_account)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match OnlineClient::<PangolinSubxtConfig>::from_url(&endpoint).await {
                Ok(client) => Ok(PangolinClient::new(client, account.clone())),
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
