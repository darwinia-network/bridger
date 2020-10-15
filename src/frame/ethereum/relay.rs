//! Ethereum Relay
use crate::chain::eth::HeaderStuff;
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelay: System {}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ConfirmedBlockNumbers<T: EthereumRelay> {
    #[store(returns = Vec<u64>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Submit proposal call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitProposal<T: EthereumRelay> {
    /// Ethereum Headerthings
    pub proposal: Vec<HeaderStuff>,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Approve pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ApprovePendingHeader<T: EthereumRelay> {
    /// pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Reject pending header call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RejectPendingHeader<T: EthereumRelay> {
    /// pending block number
    pub pending: u64,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
