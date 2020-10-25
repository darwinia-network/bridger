//! Relayer Game
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Event, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumRelayerGame: System {
    /// Ethereum Pending Header
    type PendingRelayHeaderParcel: 'static + Encode + Decode + Send + Default;
    /// Ethereum Relay Affirmation
    type RelayAffirmation: 'static + Encode + Decode + Send + Default;
    /// Ethereum Relay Header ID
    type RelayHeaderId: 'static + Encode + Decode + Send + Default + Clone + Sync;
}

/////
// Storage
/////

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
    /// map hasher(blake2_128_concat) GameId<TcBlockNumber<T, I>>
    pub map: ([u8; 32], T::RelayHeaderId),
}

/////
// Events
/////

/// A new relay parcel affirmed. [game id, round, index, relayer]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Affirmed<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
    /// Proposal Id
    pub proposal_id: u32,
    /// Proposal Round
    pub proposal_round: u32,
    /// Account Id
    pub account_id: <T as System>::AccountId,
}

/// A different affirmation submitted, dispute found. [game id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Disputed<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
}

/// An extended affirmation submitted, dispute go on. [game id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Extended<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
}

/// A game has been settled. [game id]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewRound<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
    /// Required Headers
    pub required_headers: Vec<T::RelayHeaderId>,
}

/// GameOver(RelayHeaderId),
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct GameOver<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
}

/// Approved Pending Ethereum Header Parcel
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct PendingRelayHeaderParcelApproved<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
}

/// Rejected Pending Ethereum Header Parcel
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct PendingRelayHeaderParcelRejected<T: EthereumRelayerGame> {
    /// Ethereum Relay Header Id
    pub relay_header_id: T::RelayHeaderId,
}
