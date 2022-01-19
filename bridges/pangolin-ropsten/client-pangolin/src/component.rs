use std::{thread, time};

use subxt::{Client, ClientBuilder};

use crate::config::{ClientConfig, PangolinSubxtConfig};
use crate::error::ClientError;

const MAX_ATTEMPTS: u32 = 6;

/// Subxt component
pub struct SubxtComponent;

impl SubxtComponent {
    /// Get subxt client instance
    pub async fn component(
        config: ClientConfig,
    ) -> color_eyre::Result<Client<PangolinSubxtConfig>> {
        let mut attempts = 1;
        let mut wait_secs = 1;
        let endpoint = Self::correct_url(&config.endpoint)?;
        loop {
            thread::sleep(time::Duration::from_secs(wait_secs));
            return match ClientBuilder::new().set_url(&endpoint).build().await {
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

    fn correct_url(url: impl AsRef<str>) -> color_eyre::Result<String> {
        let url = url.as_ref();
        if url.starts_with("ws://") || url.starts_with("wss://") {
            return Ok(url.to_string());
        }
        if url.starts_with("http://") || url.starts_with("https://") {
            let is_https = url.starts_with("https://");
            let fixed_url = url.replace("http://", "").replace("https://", "");
            let mut parts = fixed_url.split('/').collect::<Vec<&str>>();
            let origin_host = parts
                .first()
                .ok_or_else(|| ClientError::Other(format!("Bad url: {}", url)))?;
            let mut better_host = origin_host.to_string();
            if !origin_host.contains(':') {
                let port = if is_https { 443 } else { 80 };
                better_host = format!("{}:{}", better_host, port);
            }
            parts.remove(0);
            let better_url = format!(
                "{}{}{}",
                if is_https { "https://" } else { "http://" },
                better_host,
                parts.join("/")
            );
            tracing::trace!(target: "client-pangolin", "Correct rpc endpoint: {}", better_url);
            return Ok(better_url);
        }
        Err(ClientError::Other(format!("Wrong url: {}", url)).into())
    }
}
