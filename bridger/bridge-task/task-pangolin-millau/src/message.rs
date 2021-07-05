use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::PangolinMillauBus;
use crate::config::ChainInfoConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinMillauMessage {
    InitBridge(BridgeInfo),
}

impl Message<PangolinMillauBus> for PangolinMillauMessage {
    type Channel = broadcast::Sender<Self>;
}

// EnumFromStr
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BridgeName {
    PangolinToMillau,
    MillauToPangolin,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BridgeInfo {
    pub bridge: BridgeName,
    pub source_chain: ChainInfoConfig,
    pub target_chain: ChainInfoConfig,
}
