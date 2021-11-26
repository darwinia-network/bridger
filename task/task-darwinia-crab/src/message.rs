use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use crate::bus::DarwiniaCrabBus;
use crate::types::BridgeName;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DarwiniaCrabMessageSend {
    InitBridge(BridgeName),
    Relay,
}

impl Message<DarwiniaCrabBus> for DarwiniaCrabMessageSend {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DarwiniaCrabMessageReceive {
    FinishedInitBridge,
}

impl Message<DarwiniaCrabBus> for DarwiniaCrabMessageReceive {
    type Channel = broadcast::Sender<Self>;
}
