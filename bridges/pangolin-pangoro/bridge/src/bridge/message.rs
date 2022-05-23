use lifeline::Message;
use postage::broadcast;

use crate::bridge::BridgeTaskBus;

#[derive(Debug, Clone)]
pub enum BridgeTaskMessage {
    SomeEvent(u64),
    StopSomeService,
}

impl Message<BridgeTaskBus> for BridgeTaskMessage {
    type Channel = broadcast::Sender<Self>;
}
