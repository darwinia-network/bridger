//! Darwinia Runtime
#![cfg(feature = "runtime")]

use crate::{
    chain::{eth::EthereumHeaderThing, RelayProposal},
    frame::{
        collective::Council,
        ethereum::{backing::EthereumBacking, game::EthereumRelayerGame, relay::EthereumRelay},
        sudo::Sudo,
    },
};

use substrate_subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    sp_core,
    sp_runtime::{
        generic::Header,
        traits::{BlakeTwo256, IdentifyAccount, Verify},
        MultiSignature, OpaqueExtrinsic,
    },
    system::System,
    Runtime,
};

/// Darwinia Runtime
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DarwiniaRuntime;
impl Runtime for DarwiniaRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl Balances for DarwiniaRuntime {
    type Balance = u128;
}

impl System for DarwiniaRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Council for DarwiniaRuntime {}
impl Sudo for DarwiniaRuntime {}
impl EthereumRelay for DarwiniaRuntime {}
impl EthereumRelayerGame for DarwiniaRuntime {
    type RelayProposal = RelayProposal<
        <Self as System>::AccountId,
        <Self as Balances>::Balance,
        EthereumHeaderThing,
        <Self as System>::Hash,
    >;
    type PendingHeader = (<Self as System>::BlockNumber, u64, EthereumHeaderThing);
}
impl EthereumBacking for DarwiniaRuntime {}
