use codec::{Decode, Encode};

/// Relay Proposal
#[derive(Clone, Encode, Decode, Default)]
pub struct RelayProposal<Relayer, Balance, HeaderThing, HeaderHash> {
    /// The relayer of these series of headers
    /// The proposer of this proposal
    /// The person who support this proposal with some bonds
    pub relayer: Relayer,
    /// A series of target chain's header brief and the value that relayer had bonded for it
    pub bonded_proposal: Vec<(Balance, HeaderThing)>,
    /// Parents (previous header hash)
    ///
    /// If this field is `None` that means this proposal is the first proposal
    pub extend_from_header_hash: Option<HeaderHash>,
}
