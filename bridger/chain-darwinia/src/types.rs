use bee_client::types::substrate::system::System;
use codec::{Decode, Encode};

pub trait EthereumRelay: System {
    /// RingBalance
    type RingBalance: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum BlockNumber
    type EthereumBlockNumber: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Pending Header
    type PendingRelayHeaderParcel: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Relay Header ID
    type RelayAffirmationId: 'static + Encode + Decode + Sync + Send + Default + Clone;
}
