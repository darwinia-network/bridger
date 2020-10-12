//! Darwinia Ethereum Backing
use crate::chain::eth::{EthereumReceiptProofThing, RedeemFor};
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumBacking: System {}

/// Submit proposal call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitProposalCall<T: EthereumBacking> {
    /// Ethereum Headerthings
    pub act: RedeemFor,
    /// Ethereum Receipt Proof
    pub proof: EthereumReceiptProofThing,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
