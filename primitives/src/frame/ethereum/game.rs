//! Relayer Game
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {
    /// Ethereum Pending Header
    type PendingRelayHeaderParcel: 'static + Encode + Decode + Send + Default;
    /// Ethereum Relay Proposal
    type RelayAffirmation: 'static + Encode + Decode + Send + Default;
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingRelayHeaderParcels<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::PendingRelayHeaderParcel>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Relay Proposals Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Affirmations<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::RelayAffirmation>)]
    /// map hasher(blake2_128_concat) GameId<TcBlockNumber<T, I>>
    pub map: ([u8; 32], u64),
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
