use serde::{Deserialize, Serialize};

use crate::traits::SubqueryInfo;
use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    #[serde(default)]
    pub enable_mandatory: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub source: SI,
    pub target: SI,
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
