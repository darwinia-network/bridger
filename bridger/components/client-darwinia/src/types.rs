use bee_client::types::client::BasicSessionKeys;
use bee_client::types::client::ChainTypes;
use bee_client::types::substrate::balances::{AccountData, Balances};
use bee_client::types::substrate::events::EventTypeRegistry;
use bee_client::types::substrate::extra::DefaultExtra;
use bee_client::types::substrate::session::Session;
use bee_client::types::substrate::sudo::Sudo;
use bee_client::types::substrate::system::System;
use codec::{Decode, Encode};
use sp_runtime::generic::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
use sp_runtime::{MultiAddress, MultiSignature, OpaqueExtrinsic};

use bridge_traits::bridge::chain::{BridgeChain, ChainCategory, LikeDarwiniaChain, SubstrateChain};

pub trait EthereumRelay: System {
    /// RingBalance
    type RingBalance: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum BlockNumber
    type EthereumBlockNumber: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Pending Header
    type PendingRelayHeaderParcel: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Relay Header ID
    type RelayAffirmationId: 'static + Encode + Decode + Sync + Send + Default + Clone;
}

pub struct DarwiniaChain {}

impl BridgeChain for DarwiniaChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Substrate;
}

impl SubstrateChain for DarwiniaChain {
    type ChainTypes = DarwiniaChainTypes;
}

impl LikeDarwiniaChain for DarwiniaChain {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DarwiniaChainTypes;

impl ChainTypes for DarwiniaChainTypes {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system::<Self>();
        event_type_registry.with_balances::<Self>();
        event_type_registry.with_session::<Self>();
        // event_type_registry.with_staking();
        // event_type_registry.with_contracts();
        event_type_registry.with_sudo::<Self>();
        bee_client::types::client::register_default_type_sizes(event_type_registry);
    }
}

impl Balances for DarwiniaChainTypes {
    type Balance = u128;
}

impl System for DarwiniaChainTypes {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Session for DarwiniaChainTypes {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl Sudo for DarwiniaChainTypes {}
