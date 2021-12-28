use serde::{Deserialize, Serialize};

use component_http_client::HttpClientConfig;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateTaskConfig {
    /// Http client config
    pub http_client: HttpClientConfig,
}
