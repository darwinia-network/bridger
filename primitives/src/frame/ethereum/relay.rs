//! Ethereum Relay

use crate::chain::ethereum::{
	EthereumHeader, EthereumReceipt, EthereumRelayHeaderParcel, EthereumRelayProofs,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call, Event, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelay: System {
	/// RingBalance
	type RingBalance: 'static + Encode + Decode + Send + Default;
	/// Ethereum BlockNumber
	type EthereumBlockNumber: 'static + Encode + Decode + Send + Default;
	/// Ethereum Pending Header
	type PendingRelayHeaderParcel: 'static + Encode + Decode + Send + Default;
	/// Ethereum Relay Header ID
	type RelayAffirmationId: 'static + Encode + Decode + Send + Default + Clone + Sync;
}

//////
// Call
//////

/// Affirm Call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct Affirm<T: EthereumRelay> {
	/// Ethereum relay headr parcel
	pub ethereum_relay_header_parcel: EthereumRelayHeaderParcel,
	/// Ethereum relay proofs
	pub ethereum_relay_proofs: Option<EthereumRelayProofs>,
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// Set confirmed header parcel
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SetConfirmedParcel<T: EthereumRelay> {
	/// Ethereum relay headr parcel
	pub ethereum_relay_header_parcel: EthereumRelayHeaderParcel,
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// Approve pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct VotePendingRelayHeaderParcel<T: EthereumRelay> {
	/// pending block number
	pub block_number: u64,
	/// vote
	pub aye: bool,
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

//////
// Events
//////

/// A new relay parcel affirmed. [relayer, relay affirmation id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Affirmed<T: EthereumRelay> {
	/// Account Id
	pub account_id: <T as System>::AccountId,
	/// Ethereum Relay Header Id
	pub relay_affirmation_id: T::RelayAffirmationId,
}

/// A different affirmation submitted, dispute found. [relayer, relay affirmation id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DisputedAndAffirmed<T: EthereumRelay> {
	/// Account Id
	pub account_id: <T as System>::AccountId,
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

/// An extended affirmation submitted, dispute go on. [relayer, relay affirmation id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Extended<T: EthereumRelay> {
	/// Account Id
	pub account_id: <T as System>::AccountId,
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

/// A new round started. [game id, game sample points]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewRound<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
	/// Required Headers
	pub required_headers: Vec<T::RelayAffirmationId>,
}

/// A game has been settled. [game id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct GameOver<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

/// The specific confirmed parcel removed. [block number]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RemoveConfirmedParcel<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

/// EthereumReceipt verification. [account, ethereum receipt, ethereum header]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct VerifyReceipt<T: EthereumRelay> {
	/// The block number of Ethereum header parcel
	pub account_id: <T as System>::AccountId,
	/// Ethereum Receipt
	pub receipt: EthereumReceipt,
	/// Ethereum Header
	pub header: EthereumHeader,
}

/// Pended(EthereumBlockNumber),
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Pended<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

/// Pending relay header parcel approved. [block number, reason]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct PendingRelayHeaderParcelApproved<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
	/// reason
	pub reason: Vec<u8>,
}

/// Pending relay header parcel rejected. [block number]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct PendingRelayHeaderParcelRejected<T: EthereumRelay> {
	/// Ethereum Relay Header Id
	pub relay_header_id: T::RelayAffirmationId,
}

//////
// Store
//////

/// Pending Headers Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingRelayHeaderParcels<T: EthereumRelay> {
	#[store(returns = Vec<T::PendingRelayHeaderParcel>)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ConfirmedBlockNumbers<T: EthereumRelay> {
	#[store(returns = Vec<u64>)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}
