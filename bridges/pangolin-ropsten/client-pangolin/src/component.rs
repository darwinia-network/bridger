use std::{thread, time};

use subxt::{Client, ClientBuilder};

use crate::config::{ClientConfig, PangolinSubxtConfig};

const MAX_ATTEMPTS: u32 = 6;

/// Darwinia subxt component
pub struct SubxtComponent;

impl SubxtComponent {
    /// Get subxt client instance
    pub async fn component(
        config: ClientConfig,
    ) -> color_eyre::Result<Client<PangolinSubxtConfig>> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match ClientBuilder::new()
                .set_url(config.endpoint.clone())
                .build()
                .await
            {
                Ok(client) => Ok(client),
                Err(err) => {
                    if attempts < MAX_ATTEMPTS {
                        attempts += 1;
                        wait_secs *= 2; // backoff
                        continue;
                    }
                    Err(err)?
                }
            };
        }
    }
}
