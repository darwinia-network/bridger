use std::{thread, time};

use subxt::ClientBuilder;

use crate::client::DarwiniaClient;
use crate::config::ClientConfig;
use crate::error::{ClientError, ClientResult};
use crate::types::DarwiniaAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct DarwiniaClientComponent;

impl DarwiniaClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<DarwiniaClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account =
            DarwiniaAccount::new(config.relayer_private_key, config.relayer_real_account)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match ClientBuilder::new().set_url(&endpoint).build().await {
                Ok(client) => Ok(DarwiniaClient::new(client, account.clone())),
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
