use serde::{Deserialize, Serialize};

/// Http client config
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HttpClientVendor {
    /// The timeout of request
    pub timeout: u64,
}

impl HttpClientVendor {
    /// Get request client
    pub fn component(&self) -> color_eyre::Result<reqwest::Client> {
        Ok(reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(self.timeout))
            .build()?)
    }
}
