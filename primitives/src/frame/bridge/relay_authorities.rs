//! Darwinia Bridge Relay Authorities
use codec::{Encode, Decode};
use substrate_subxt::{system::{System, SystemEventsDecoder}};
use substrate_subxt_proc_macro::{module, Event, Store, Call};
use frame_support::sp_runtime::app_crypto::sp_core::H256;
use core::marker::PhantomData;
use crate::runtime::EcdsaSignature;

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
    /// EcdsaAddress
    type EcdsaAddress: 'static + Encode + Decode + Send + Default;
    /// Relay signature
    type RelaySignature: 'static + Encode + Decode + Send + Sync + Default;
}

//////
// Calls
//////

/// Submit authorities signature
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedAuthorities<T: EthereumRelayAuthorities> {
    /// Runtime marker
    pub _runtime: PhantomData<T>,
    /// signature
    pub signature: T::RelaySignature,
}

/// Submit redeem call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedMmrRoot<T: EthereumRelayAuthorities> {
    /// block_number
    pub block_number: <T as System>::BlockNumber,
    /// mmr_root
    pub mmr_root: H256,
    /// signature
    pub signature: T::RelaySignature
}

//////
// Events
//////

/// A New MMR Root Request to be Signed. [block number]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewMMRRoot<T: EthereumRelayAuthorities> {
    /// BlockNumber
    pub block_number: <T as System>::BlockNumber,
}

/// Authorities Signed. [term, message, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AuthoritiesSetSigned<T: EthereumRelayAuthorities> {
    /// term
    pub term: u32,
    /// message
    pub message: Vec<u8>,
    /// signatures
    pub signatures: Vec<(<T as System>::AccountId, EcdsaSignature)>,
}

/// MMR Root Signed. [block number, mmr root, message, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MMRRootSigned<T: EthereumRelayAuthorities> {
    /// block number
    pub block_number: u128,
    /// mmr root
    pub mmr_root: H256,
    /// message
    pub message: Vec<u8>,
    /// The redeemed balance
    pub signatures: Vec<(<T as System>::AccountId, EcdsaSignature)>,
}

/// NewAuthorities. [message to sign]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewAuthorities<T: EthereumRelayAuthorities> {
    /// message
    pub message: Vec<u8>,
    /// marker
    pub _marker: PhantomData<T>,
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
