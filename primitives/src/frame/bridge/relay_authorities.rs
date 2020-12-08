//! Darwinia Bridge Relay Authorities
use codec::Decode;
use substrate_subxt::{system::{System, SystemEventsDecoder}};
use substrate_subxt_proc_macro::{module, Event};
use frame_support::sp_runtime::app_crypto::sp_core::H256;

/// Bridge Relay Authorities Pallet
#[module]
pub trait RelayAuthorities: System {
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
pub struct SignedAuthoritySet<T: RelayAuthorities> {
    /// new_authorities
    pub new_authorities: Vec<u8>,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, RelaySignature)>,
}

/// SignedMMRRoot. [mmr_root, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SignedMMRRoot<T: RelayAuthorities> {
    /// mmr root
    pub mmr_root: H256,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, RelaySignature)>,
}

//////
// Store
//////

