use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::PangolinMillauBus;

pub type BridgeName = support_s2s::types::BridgeName;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinMillauMessageSend {
    InitBridge(BridgeName),
    Relay(BridgeName),
}

impl Message<PangolinMillauBus> for PangolinMillauMessageSend {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinMillauMessageReceive {
    FinishedInitBridge,
}

impl Message<PangolinMillauBus> for PangolinMillauMessageReceive {
    type Channel = broadcast::Sender<Self>;
}
