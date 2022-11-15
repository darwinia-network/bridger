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
    CSI: S2SParaBridgeSoloChainInfo,
    CRI: S2SParaBridgeRelayChainInfo,
    CPI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    /// Chain config
    pub chain: ChainConfig<CSI, CRI, CPI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Storage for BridgeConfig<CSI, CRI, CPI, SI>
{
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > lifeline::Resource<BridgeBus> for BridgeConfig<CSI, CRI, CPI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<
    CSI: S2SParaBridgeSoloChainInfo,
    CRI: S2SParaBridgeRelayChainInfo,
    CPI: S2SSoloBridgeSoloChainInfo,
> {
    /// Solo chain
    pub solo: CSI,
    /// Para chain
    pub para: CPI,
    /// Relay chain
    pub relay: CRI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub solo: SI,
    pub para: SI,
    pub relay: SI,
}
