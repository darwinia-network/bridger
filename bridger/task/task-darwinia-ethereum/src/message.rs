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
    StartRelay,
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

// *** ToExtrinsicsMessage **
#[derive(Clone, Debug)]
pub enum ToExtrinsicsMessage {
	Extrinsic(Extrinsic),
}

use bridge_primitives::chain::ethereum::{
    EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor,
};

pub type EcdsaMessage = [u8; 32];
#[derive(Clone, Debug)]
pub enum Extrinsic {
	Affirm(EthereumRelayHeaderParcel),
	Redeem(RedeemFor, EthereumReceiptProofThing, EthereumTransaction),
	GuardVote(u64, bool),
	SignAndSendMmrRoot(u32),
	SignAndSendAuthorities(EcdsaMessage),
}

impl Message<DarwiniaEthereumBus> for ToExtrinsicsMessage {
    type Channel = broadcast::Sender<Self>;
}