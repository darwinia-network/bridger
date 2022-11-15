use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::config::RelayConfig;
use crate::bridge::BridgeBus;
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

/// Solo with solo bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> {
    /// Chain config
    pub chain: ChainConfig<CI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> Storage for BridgeConfig<CI, SI> {
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> lifeline::Resource<BridgeBus>
    for BridgeConfig<CI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<CI: S2SSoloBridgeSoloChainInfo> {
    /// Source chain
    pub source: CI,
    /// Target chain
    pub target: CI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub source: SI,
    pub target: SI,
}
