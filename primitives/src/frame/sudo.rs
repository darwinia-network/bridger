//! Darwinia Sudo
use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::weights::Weight;
use substrate_subxt::{
    system::{System, SystemEventsDecoder},
    Encoded,
};
use substrate_subxt_proc_macro::{module, Call, Store};

/// The subset of the `frame_sudo::Trait` that a client must implement.
#[module]
pub trait Sudo: System {}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SudoCall<'a, T: Sudo> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Encoded transaction.
    pub call: &'a Encoded,
}

/// Get the sudo AccountId
#[derive(Clone, Debug, Eq, PartialEq, Store, Decode, Encode)]
pub struct Key<T: Sudo> {
    #[store(returns = <T as System>::AccountId)]
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

/// Execute a transaction with sudo permissions without checking the call weight.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SudoUncheckedWeightCall<'a, T: Sudo> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Encoded transaction.
    pub call: &'a Encoded,
    /// Call weight.
    ///
    /// This argument is actually unused in runtime, you can pass any value of
    /// `Weight` type when using this call.
    pub weight: Weight,
}
