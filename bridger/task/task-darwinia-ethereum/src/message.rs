#![allow(dead_code)]

use std::fmt::Debug;

use lifeline::Message;
use postage::broadcast;

use crate::bus::DarwiniaEthereumBus;

use crate::service::EthereumTransaction;

#[derive(Debug, Clone)]
pub enum DarwiniaEthereumMessage {
    Scan(EthereumScanMessage),
    ToDarwinia(ToDarwiniaLinkedMessage),
}

impl Message<DarwiniaEthereumBus> for DarwiniaEthereumMessage {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone)]
pub enum EthereumScanMessage {
    Start,
    Pause,
}

#[derive(Debug, Clone)]
pub enum ToDarwiniaLinkedMessage {
    SendExtrinsic,
}

// *** ToRelayMessage ***
#[derive(Clone, Debug)]
pub enum ToRelayMessage {
	EthereumBlockNumber(u64),
}

impl Message<DarwiniaEthereumBus> for ToRelayMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToRedeemMessage **
#[derive(Clone, Debug)]
pub enum ToRedeemMessage {
	EthereumTransaction(EthereumTransaction),
}

impl Message<DarwiniaEthereumBus> for ToRedeemMessage {
    type Channel = broadcast::Sender<Self>;
}