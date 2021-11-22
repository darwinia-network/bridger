use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubscanConfig {
    pub endpoint: String,
    pub token: String,
    pub timeout: Option<u64>,
}

impl BridgeConfig for SubscanConfig {
    fn marker() -> &'static str {
        "component-subscan"
    }

    fn template() -> Self {
        Self {
            endpoint: "https://darwinia.api.subscan.io".to_string(),
            token: "123456".to_string(),
            timeout: Some(3000),
        }
    }
}
