use lifeline::Message;
use postage::broadcast;

use crate::bus::DarwiniaEthereumBus;

#[derive(Debug, Clone)]
pub enum ToDarwiniaLinkedMessage {
    SendExtrinsic,
}

impl Message<DarwiniaEthereumBus> for ToDarwiniaLinkedMessage {
    type Channel = broadcast::Sender<Self>;
}
