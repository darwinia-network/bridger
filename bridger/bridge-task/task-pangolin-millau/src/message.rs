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

#[derive(Debug, Clone, EnumFromStr)]
pub enum BridgeName {
    PangolinToMillau,
    MillauToPangolin,
}
