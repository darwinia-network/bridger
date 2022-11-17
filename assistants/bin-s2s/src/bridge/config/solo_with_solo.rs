use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::config::RelayConfig;
use crate::bridge::BridgeBus;
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

/// Solo with solo bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<
    SCI: S2SSoloBridgeSoloChainInfo,
    TCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    /// Chain config
    pub chain: ChainConfig<SCI, TCI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> Storage
    for BridgeConfig<SCI, TCI, SI>
{
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    lifeline::Resource<BridgeBus> for BridgeConfig<SCI, TCI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo> {
    /// Source chain
    pub source: SCI,
    /// Target chain
    pub target: TCI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub source: SI,
    pub target: SI,
}
