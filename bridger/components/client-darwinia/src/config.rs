use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DarwiniaConfig {
    pub endpoint: String,
    pub strict: bool,
}

impl BridgeConfig for DarwiniaConfig {
    fn marker() -> &'static str {
        "component-darwinia"
    }
}
