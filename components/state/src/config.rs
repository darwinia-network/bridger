use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BridgeStateConfig {
    pub microkv: MicrokvConfig,
}

impl BridgeConfig for BridgeStateConfig {
    fn marker() -> &'static str {
        "component-bridge-state"
    }

    fn template() -> Self {
        Self {
            microkv: MicrokvConfig::template(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MicrokvConfig {
    pub base_path: PathBuf,
    pub db_name: Option<String>,
    pub auto_commit: bool,
}

impl BridgeConfig for MicrokvConfig {
    fn marker() -> &'static str {
        "component-microkv"
    }

    fn template() -> Self {
        Self {
            base_path: "/tmp/microkv".into(),
            db_name: Some("microkv".to_string()),
            auto_commit: true,
        }
    }
}
