use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::{BridgeBus, IndexConfig, RelayConfig};
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};

/// Solo with solo bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
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
