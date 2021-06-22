use lifeline::Message;
use postage::mpsc;

use crate::bus::BridgeBus;

#[derive(Debug, Clone)]
pub enum EthereumMessage {
    Confirmed(u64),
}

impl Message<BridgeBus> for EthereumMessage {
    type Channel = mpsc::Sender<Self>;
}
