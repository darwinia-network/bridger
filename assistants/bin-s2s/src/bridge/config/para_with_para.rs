use lifeline::Storage;
use serde::{Deserialize, Serialize};

use crate::bridge::config::{ParaWithParaConfig, RelayConfig};
use crate::bridge::BridgeBus;
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

/// Para with para bridge config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<
    SSCI: S2SParaBridgeSoloChainInfo,
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SSoloBridgeSoloChainInfo,
    TSCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    /// Chain config
    pub chain: ChainConfig<SSCI, SRCI, SPCI, TSCI, TRCI, TPCI>,
    /// Relay config
    pub relay: RelayConfig,
    /// Para config
    pub para_config: ParaWithParaConfig,
    /// Index config
    pub index: IndexConfig<SI>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainConfig<
    SSCI: S2SParaBridgeSoloChainInfo,
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SSoloBridgeSoloChainInfo,
    TSCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SSoloBridgeSoloChainInfo,
> {
    /// Source Solo chain
    pub source_solo: SSCI,
    /// Source Para chain
    pub source_para: SPCI,
    /// Source Relay chain
    pub source_relay: SRCI,
    /// Target Solo chain
    pub target_solo: TSCI,
    /// Target Para chain
    pub target_para: TPCI,
    /// Target Relay chain
    pub target_relay: TRCI,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig<SI: SubqueryInfo> {
    pub source_solo: SI,
    pub source_para: SI,
    pub source_relay: SI,
    pub target_solo: SI,
    pub target_para: SI,
    pub target_relay: SI,
}
