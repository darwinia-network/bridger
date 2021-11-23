use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::DarwiniaCrabBus;
use crate::types::BridgeName;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinPangoroMessageSend {
    InitBridge(BridgeName),
    Relay,
}

impl Message<DarwiniaCrabBus> for PangolinPangoroMessageSend {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PangolinPangoroMessageReceive {
    FinishedInitBridge,
}

impl Message<DarwiniaCrabBus> for PangolinPangoroMessageReceive {
    type Channel = broadcast::Sender<Self>;
}
