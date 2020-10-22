use codec::{Decode, Encode};

/// Game id, round and the index under the round point to a unique affirmation AKA affirmation id
#[derive(Clone, PartialEq, Encode, Decode, Default)]
pub struct RelayAffirmationId<RelayHeaderId> {
    /// Relay header id aka game id
    pub relay_header_id: RelayHeaderId,
    /// Round index
    pub round: u32,
    /// Index of a affirmation list which under a round
    pub index: u32,
}

/// Relay Affirmations
#[derive(Clone, Encode, Decode, Default)]
pub struct RelayAffirmation<RelayHeaderParcel, Relayer, Balance, RelayHeaderId> {
    /// Relayer
    pub relayer: Relayer,
    /// Relay header parcels
    pub relay_header_parcels: Vec<RelayHeaderParcel>,
    /// Stake balance
    pub stake: Balance,
    /// Affirmation ID
    pub maybe_extended_relay_affirmation_id: Option<RelayAffirmationId<RelayHeaderId>>,
    /// Verified
    pub verified_on_chain: bool,
}
