use serde::{Deserialize, Serialize};

/// Http client config
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HttpClientConfig {
    /// The timeout of request
    pub timeout: u64,
}
