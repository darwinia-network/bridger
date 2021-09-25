use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::PangolinPangoroBus;
use crate::types::BridgeName;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinPangoroMessageSend {
    InitBridge(BridgeName),
    Relay,
}

impl Message<PangolinPangoroBus> for PangolinPangoroMessageSend {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinPangoroMessageReceive {
    FinishedInitBridge,
}

impl Message<PangolinPangoroBus> for PangolinPangoroMessageReceive {
    type Channel = broadcast::Sender<Self>;
}
