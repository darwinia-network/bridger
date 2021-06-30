use lifeline::Message;
use postage::broadcast;

use crate::bus::SharedBus;

#[derive(Debug, Clone)]
pub enum SharedMessage {
    Darwinia(DarwiniaMessage),
}

impl Message<SharedBus> for SharedMessage {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone)]
pub enum DarwiniaMessage {
    SendExtrinsic,
}
