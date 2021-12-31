use serde::{Deserialize, Serialize};

/// Shadow config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub endpoint: String,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://shadow.darwinia.network".to_string(),
        }
    }
}
