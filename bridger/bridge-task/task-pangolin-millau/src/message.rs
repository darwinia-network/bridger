use lifeline::Message;
use postage::broadcast;

use crate::bus::PangolinMillauBus;

#[derive(Debug, Clone)]
pub enum PangolinMillauMessage {
    InitBridge(BridgeName),
}

impl Message<PangolinMillauBus> for PangolinMillauMessage {
    type Channel = broadcast::Sender<Self>;
}

// EnumFromStr
#[derive(Debug, Clone)]
pub enum BridgeName {
    PangolinToMillau,
    MillauToPangolin,
}
