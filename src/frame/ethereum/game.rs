//! Relayer Game
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {
    /// Ethereum Pending Header
    type PendingHeader: 'static + Encode + Decode + Send + Default;
    /// Ethereum Relay Proposal
    type RelayProposal: 'static + Encode + Decode + Send + Default;
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingHeaders<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::PendingHeader>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Relay Proposals Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Proposals<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::RelayProposal>)]
    /// map hasher(blake2_128_concat) GameId<TcBlockNumber<T, I>>
    pub map: ([u8; 32], u64),
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
