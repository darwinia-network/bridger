use std::fmt::Debug;

use lifeline::Message;
use postage::broadcast;

use crate::bus::DarwiniaEthereumBus;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EthereumScanMessage {
    Start,
    Pause,
}

impl Message<DarwiniaEthereumBus> for EthereumScanMessage {
    type Channel = broadcast::Sender<Self>;
}
