use serde::{Deserialize, Serialize};

use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    #[serde(default)]
    pub enable_mandatory: bool,
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}

#[cfg(feature = "solo-with-para")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SoloWithParaConfig {
    pub para_id: u32,
}

#[cfg(feature = "para-with-para")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParaWithParaConfig {
    pub source_para_id: u32,
    pub target_para_id: u32,
}
