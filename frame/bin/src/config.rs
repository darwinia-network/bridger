use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;

use crate::command::types::RegistryType;

/// Bridger config
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BridgerConfig {
    /// The registry of bridger
    pub registry: BridgerRegistry,
}

/// Bridger registry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgerRegistry {
    /// Registry type
    #[serde(rename = "type")]
    pub type_: RegistryType,
    /// The path of registry
    #[serde(with = "string_empty_as_none")]
    pub path: Option<String>,
    /// The version of bridge
    #[serde(with = "string_empty_as_none")]
    pub version: Option<String>,
}

impl Default for BridgerRegistry {
    fn default() -> Self {
        Self {
            type_: RegistryType::Github,
            path: Some("https://github.com/darwinia-network/bridger".to_string()),
            version: None,
        }
    }
}
