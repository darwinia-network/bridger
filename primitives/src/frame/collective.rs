//! Darwinia Collective

use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::{
    system::{System, SystemEventsDecoder},
    Encoded,
};
use substrate_subxt_proc_macro::{module, Call, Store};

/// The subset of the `frame_council::Trait` that a client must implement.
#[module]
pub trait Council: System {}

/// Get the sudo AccountId
#[derive(Clone, Debug, Eq, PartialEq, Store, Decode, Encode)]
pub struct Members<T: Council> {
    #[store(returns = Vec<<T as System>::AccountId>)]
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct Execute<'a, T: Council> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Encoded transaction.
    pub proposal: &'a Encoded,
    /// Proposal length.
    pub bound: u32,
}
