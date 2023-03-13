use std::{thread, time};

use subxt::OnlineClient;

use crate::client::RococoClient;
use crate::config::{ClientConfig, RococoSubxtConfig};
use crate::error::ClientResult;
use crate::types::RococoAccount;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct RococoClientComponent;

impl RococoClientComponent {
    /// Get subxt client instance
    pub async fn component(config: ClientConfig) -> ClientResult<RococoClient> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = support_toolkit::url::correct_endpoint(&config.endpoint)?;
        let account = RococoAccount::new(config.relayer_private_key)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match OnlineClient::<RococoSubxtConfig>::from_url(&endpoint).await {
                Ok(client) => Ok(RococoClient::new(client, account.clone())),
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
