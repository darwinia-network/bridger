#![allow(dead_code)]

use std::fmt::Debug;

use lifeline::Message;
use postage::broadcast;

use component_thegraph_liketh::types::TransactionEntity;
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::receipt::EthereumReceiptProofThing;

use crate::bus::DarwiniaEthereumBus;

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
    Stop,
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
    EthereumTransaction(TransactionEntity),
}

impl Message<DarwiniaEthereumBus> for ToRedeemMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToExtrinsicsMessage **
#[derive(Clone, Debug)]
pub enum ToExtrinsicsMessage {
    Extrinsic(Extrinsic),
}

pub type EcdsaMessage = [u8; 32];
#[derive(Clone, Debug)]
pub enum Extrinsic {
    Affirm(EthereumRelayHeaderParcel),
    Redeem(EthereumReceiptProofThing, TransactionEntity),
    GuardVote(u64, bool),
    SignAndSendMmrRoot(u32),
    SignAndSendAuthorities(EcdsaMessage),
}

impl Message<DarwiniaEthereumBus> for ToExtrinsicsMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToGuardMessage **
#[derive(Clone, Debug)]
pub enum ToGuardMessage {
    StartGuard,
}

impl Message<DarwiniaEthereumBus> for ToGuardMessage {
    type Channel = broadcast::Sender<Self>;
}
