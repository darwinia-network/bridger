//! Relayer Game
use core::marker::PhantomData;

use codec::{Decode, Encode};
use substrate_subxt::system::System;
use substrate_subxt_proc_macro::{module, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {
    /// Ethereum Relay Affirmation
    type RelayAffirmation: 'static + Encode + Decode + Sync + Send + Default;
}

//////
// Storage
//////

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
