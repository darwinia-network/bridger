use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::config::RelayConfig;
use crate::bridge::BridgeBus;
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

/// Solo with solo bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<
    SCI: S2SParaBridgeSoloChainInfo,
    RCI: S2SParaBridgeRelayChainInfo,
    PCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    /// Chain config
    pub chain: ChainConfig<SCI, RCI, PCI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Storage for BridgeConfig<SCI, RCI, PCI, SI>
{
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > lifeline::Resource<BridgeBus> for BridgeConfig<SCI, RCI, PCI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<
    SCI: S2SParaBridgeSoloChainInfo,
    RCI: S2SParaBridgeRelayChainInfo,
    PCI: S2SSoloBridgeSoloChainInfo,
> {
    /// Solo chain
    pub solo: SCI,
    /// Para chain
    pub para: PCI,
    /// Relay chain
    pub relay: RCI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub solo: SI,
    pub para: SI,
    pub relay: SI,
}
