use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub endpoint: String,
}

impl BridgeConfig for ShadowConfig {
    fn marker() -> &'static str {
        "component-shadow"
    }

    fn template() -> Self {
        Self {
            endpoint: "https://shadow.darwinia.network".to_string(),
        }
    }
}
