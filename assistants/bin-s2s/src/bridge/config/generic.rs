use serde::{Deserialize, Serialize};

use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    #[serde(default)]
    pub enable_mandatory: bool,
    #[cfg(any(feature = "solo-with-para", feature = "para-with-para"))]
    pub para_id: u32,
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
