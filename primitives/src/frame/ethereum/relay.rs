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
pub trait EthereumRelay: System {}

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
pub struct ApprovePendingRelayHeaderParcel<T: EthereumRelay> {
    /// pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Reject pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RejectPendingRelayHeaderParcel<T: EthereumRelay> {
    /// Pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

//////
// Event
//////
/// Remove confirmed parcel
#[derive(Clone, Debug, PartialEq, Event, Decode)]
pub struct RemoveConfirmedParcel<T: EthereumRelay> {
    /// The block number of Ethereum header parcel
    pub block: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Remove confirmed parcel
#[derive(Clone, Debug, PartialEq, Event, Decode)]
pub struct VerifyReceipt<T: EthereumRelay> {
    /// The block number of Ethereum header parcel
    pub account_id: <T as System>::AccountId,
    /// Ethereum Receipt
    pub receipt: EthereumReceipt,
    /// Ethereum Header
    pub header: EthereumHeader,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

//////
// Store
//////

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ConfirmedBlockNumbers<T: EthereumRelay> {
    #[store(returns = Vec<u64>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
