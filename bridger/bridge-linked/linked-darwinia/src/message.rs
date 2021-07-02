use lifeline::Message;
use postage::broadcast;

use crate::bus::DarwiniaLinkedBus;

#[derive(Debug, Clone)]
pub enum DarwiniaLinkedMessage {
    SendExtrinsic,
}

impl Message<DarwiniaLinkedBus> for DarwiniaLinkedMessage {
    type Channel = broadcast::Sender<Self>;
}
