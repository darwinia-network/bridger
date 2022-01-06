use serde::{Deserialize, Serialize};

/// Http client config
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HttpClientConfig {
    /// The timeout of request
    pub timeout: u64,
}

/// Http client component
pub struct HttpClientComponent;

impl HttpClientComponent {
    /// Get request client
    pub fn component(config: HttpClientConfig) -> color_eyre::Result<reqwest::Client> {
        Ok(reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?)
    }
}
