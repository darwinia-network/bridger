use lifeline::Message;
use postage::broadcast;

use crate::bus::DarwiniaLinkedBus;

#[derive(Debug, Clone)]
pub enum DarwiniaLinkedMessage {
    SendExtrinsic,
    TestRoute,
}

impl Message<DarwiniaLinkedBus> for DarwiniaLinkedMessage {
    type Channel = broadcast::Sender<Self>;
}
