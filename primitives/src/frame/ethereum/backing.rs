//! Darwinia Ethereum Backing
use crate::chain::ethereum::{EthereumReceiptProofThing, RedeemFor};
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumBacking: System {}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct VerifiedProof<T: EthereumBacking> {
    #[store(returns = Option<bool>)]
    /// Receipt tx hash
    pub map: ([u8; 32], u64),
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Submit proposal call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct Redeem<T: EthereumBacking> {
    /// Token type
    pub act: RedeemFor,
    /// Ethereum Receipt Proof
    pub proof: EthereumReceiptProofThing,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
