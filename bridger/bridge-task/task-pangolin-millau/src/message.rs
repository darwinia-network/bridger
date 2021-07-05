use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::PangolinMillauBus;

pub type BridgeName = external_s2s::types::BridgeName;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinMillauMessage {
    InitBridge(BridgeName),
    Relay(BridgeName),
}

impl Message<PangolinMillauBus> for PangolinMillauMessage {
    type Channel = broadcast::Sender<Self>;
}
