use serde::{Deserialize, Serialize};
use support_config::types::HttpClientConfig;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateTaskConfig {
    /// Http client config
    pub http_client: HttpClientConfig,
}
