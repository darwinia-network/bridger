use serde::{Deserialize, Serialize};

use component_http_client::HttpClientVendor;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateTaskConfig {
    /// Http client config
    pub http_client: HttpClientVendor,
}
