use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::config::{ParaWithParaConfig, RelayConfig};
use crate::bridge::BridgeBus;
use crate::traits::{S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, SubqueryInfo};

/// Para with para bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SParaBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    /// Chain config
    pub chain: ChainConfig<SRCI, SPCI, TRCI, TPCI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Para config
    pub para_config: ParaWithParaConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Storage for BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>
{
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > lifeline::Resource<BridgeBus> for BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>
{
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SParaBridgeSoloChainInfo,
> {
    /// Source Para chain
    pub source_para: SPCI,
    /// Source Relay chain
    pub source_relay: SRCI,
    /// Target Para chain
    pub target_para: TPCI,
    /// Target Relay chain
    pub target_relay: TRCI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub source_para: SI,
    pub source_relay: SI,
    pub target_para: SI,
    pub target_relay: SI,
}
