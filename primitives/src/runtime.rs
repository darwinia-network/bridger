//! Darwinia Runtime
#![cfg(feature = "runtime")]

use crate::{
    chain::{
        ethereum::EthereumRelayHeaderParcel, proxy_type::ProxyType, RelayAffirmation,
        RelayAffirmationId, RelayVotingState,
    },
    frame::{
        ethereum::{backing::EthereumBacking, game::EthereumRelayerGame, relay::EthereumRelay},
        proxy::Proxy,
        sudo::Sudo,
        technical_committee::TechnicalCommittee,
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

impl TechnicalCommittee for DarwiniaRuntime {}
impl Sudo for DarwiniaRuntime {}
impl EthereumRelay for DarwiniaRuntime {
    type RingBalance = u128;
    type EthereumBlockNumber = u64;
    type PendingRelayHeaderParcel = (
        <Self as System>::BlockNumber,
        EthereumRelayHeaderParcel,
        RelayVotingState<<Self as System>::AccountId>,
    );
    type RelayAffirmationId = RelayAffirmationId<Self::EthereumBlockNumber>;
}

impl EthereumRelayerGame for DarwiniaRuntime {
    type RelayAffirmation = RelayAffirmation<
        EthereumRelayHeaderParcel,
        <Self as System>::AccountId,
        <Self as Balances>::Balance,
        RelayAffirmationId<u64>,
    >;
}

impl EthereumBacking for DarwiniaRuntime {
    type EthereumTransactionIdex = u64;
}

impl Proxy for DarwiniaRuntime {
    type ProxyType = ProxyType;
}
