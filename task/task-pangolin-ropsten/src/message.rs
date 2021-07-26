#![allow(dead_code)]

use std::fmt::Debug;

use lifeline::Message;
use postage::broadcast;

use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::receipt::{EthereumReceiptProofThing, RedeemFor};

use crate::bus::PangolinRopstenBus;
use crate::service::EthereumTransaction;

#[derive(Debug, Clone)]
pub enum DarwiniaEthereumMessage {
    Scan(EthereumScanMessage),
    ToDarwinia(ToDarwiniaLinkedMessage),
}

impl Message<PangolinRopstenBus> for DarwiniaEthereumMessage {
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
    Relay,
}

impl Message<PangolinRopstenBus> for ToRelayMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToRedeemMessage **
#[derive(Clone, Debug)]
pub enum ToRedeemMessage {
    EthereumTransaction(EthereumTransaction),
}

impl Message<PangolinRopstenBus> for ToRedeemMessage {
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
    Redeem(RedeemFor, EthereumReceiptProofThing, EthereumTransaction),
    GuardVote(u64, bool),
    SignAndSendMmrRoot(u32),
    SignAndSendAuthorities(EcdsaMessage),
}

impl Message<PangolinRopstenBus> for ToExtrinsicsMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToGuardMessage **
#[derive(Clone, Debug)]
pub enum ToGuardMessage {
    StartGuard,
}

impl Message<PangolinRopstenBus> for ToGuardMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToDarwiniaMessage **
#[derive(Clone, Debug)]
pub enum ToDarwiniaMessage {
    Start,
    Stop,
}

impl Message<PangolinRopstenBus> for ToDarwiniaMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToEthereumMessage **
#[derive(Clone, Debug)]
pub enum ToEthereumMessage {
    Start,
    Stop,
}

impl Message<PangolinRopstenBus> for ToEthereumMessage {
    type Channel = broadcast::Sender<Self>;
}

