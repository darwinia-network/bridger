//! Relayer Game
use crate::chain::eth::PendingHeader;
use codec::Encode;
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingHeadersStore<T: EthereumRelayerGame> {
    #[store(returns = Vec<PendingHeader>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
