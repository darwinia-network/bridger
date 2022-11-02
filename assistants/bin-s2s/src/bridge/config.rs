use lifeline::Storage;
use serde::{Deserialize, Serialize};

use support_types::mark::BridgeName;

use crate::bridge::BridgeBus;
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};
use crate::types::HexLaneId;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
    pub bridge_name: BridgeName,
    /// Chain config
    pub chain: ChainConfig<CI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> Storage for BridgeConfig<CI, SI> {
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> lifeline::Resource<BridgeBus>
    for BridgeConfig<CI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<CI: S2SSoloChainInfo> {
    /// Source chain
    pub source: CI,
    /// Target chain
    pub target: CI,
}

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

// impl IndexConfig {
//     pub fn to_crab_subquery(&self) -> Subquery {
//         SubqueryComponent::component(self.crab.clone(), BridgeName::DarwiniaCrab)
//     }
//
//     pub fn to_darwinia_subquery(&self) -> Subquery {
//         SubqueryComponent::component(self.darwinia.clone(), BridgeName::DarwiniaCrab)
//     }
// }

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
