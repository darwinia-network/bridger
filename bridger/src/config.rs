use crate::command::types::RegistryType;
use serde::{Deserialize, Serialize};

/// Bridger config
#[derive(Clone, Debug, Deserialize, Serialize)]
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
}
