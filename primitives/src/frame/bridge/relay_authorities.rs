//! Darwinia Bridge Relay Authorities
use codec::{Encode, Decode};
use substrate_subxt::{system::{System, SystemEventsDecoder}};
use substrate_subxt_proc_macro::{module, Event, Store};
use frame_support::sp_runtime::app_crypto::sp_core::H256;
use core::marker::PhantomData;

/// Relay Authority
#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct RelayAuthority<AccountId, Signer, RingBalance, BlockNumber> {
    /// account_id
    pub account_id: AccountId,
    /// signer
    pub signer: Signer,
    /// Stake balance
    pub stake: RingBalance,
    /// BlockNumber
    pub term: BlockNumber,
}

/// Bridge Relay Authorities Pallet
#[module]
pub trait EthereumRelayAuthorities: System {
    /// Relay Authority
    type RelayAuthority: 'static + Encode + Decode + Send + Default;
}

//////
// Calls
//////


//////
// Events
//////

/// EcdsaSignature
pub type RelaySignature = [u8; 65];

/// SignedAuthoritySet. [new_authorities, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SignedAuthoritySet<T: EthereumRelayAuthorities> {
    /// new_authorities
    pub new_authorities: Vec<u8>,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, RelaySignature)>,
}

/// SignedMMRRoot. [mmr_root, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SignedMMRRoot<T: EthereumRelayAuthorities> {
    /// mmr root
    pub mmr_root: H256,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, RelaySignature)>,
}

//////
// Store
//////

/// Relay Authorities Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Authorities<T: EthereumRelayAuthorities> {
    #[store(returns = Vec<T::RelayAuthority>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
