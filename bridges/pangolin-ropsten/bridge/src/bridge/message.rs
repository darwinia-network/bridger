#![allow(dead_code)]

use std::fmt::Debug;

use lifeline::Message;
use postage::broadcast;
use serde::{Deserialize, Serialize};

use client_pangolin::types::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use client_pangolin::types::{EcdsaMessage, EthereumReceiptProofThing};
use thegraph_liketh::types::TransactionEntity;

use crate::bridge::PangolinRopstenBus;

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
}

impl Message<PangolinRopstenBus> for ToRelayMessage {
    type Channel = broadcast::Sender<Self>;
}

// *** ToExtrinsicsMessage **
#[derive(Clone, Debug)]
pub enum ToExtrinsicsMessage {
    Extrinsic(Extrinsic),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Extrinsic {
    Affirm(EthereumRelayHeaderParcel),
    Redeem(EthereumReceiptProofThing, TransactionEntity),
    GuardVote(u64, bool),
    SignAndSendMmrRoot(u32),
    SignAndSendAuthorities(EcdsaMessage),
}

impl PartialEq for Extrinsic {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Extrinsic::Affirm(left), Extrinsic::Affirm(right)) => {
                left.header.number == right.header.number
            }
            (Extrinsic::Redeem(_, l_entity), Extrinsic::Redeem(_, r_entity)) => {
                l_entity.tx_hash == r_entity.tx_hash
            }
            (Extrinsic::GuardVote(l_u, l_b), Extrinsic::GuardVote(r_u, r_b)) => {
                l_u == r_u && l_b == r_b
            }
            (Extrinsic::SignAndSendMmrRoot(left), Extrinsic::SignAndSendMmrRoot(right)) => {
                left == right
            }
            (Extrinsic::SignAndSendAuthorities(left), Extrinsic::SignAndSendAuthorities(right)) => {
                left == right
            }
            (_, _) => false,
        }
    }
}

impl Eq for Extrinsic {}

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
