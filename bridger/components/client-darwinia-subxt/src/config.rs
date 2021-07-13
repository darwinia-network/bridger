use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaSubxtConfig {
    pub endpoint: String,
}

impl BridgeConfig for DarwiniaSubxtConfig {
    fn marker() -> &'static str {
        "component-darwinia-subxt"
    }

    fn template() -> Self {
        Self {
            endpoint: "wss://rpc.darwinia.network".to_string(),
        }
    }
}
