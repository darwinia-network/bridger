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
    /// Ethereum Relay Affirmation
    type RelayAffirmation: 'static + Encode + Decode + Send + Default;
}

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct PendingRelayHeaderParcels<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::PendingRelayHeaderParcel>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Relay Affirmations Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Affirmations<T: EthereumRelayerGame> {
    #[store(returns = Vec<T::RelayAffirmation>)]
    /// game id
    pub game_id: u64,
    /// round id
    pub round_id: u32,
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
