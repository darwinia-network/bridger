use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CandidateIncludedEvent {
    pub id: String,

    /// block number of relay chain
    #[serde(rename = "includedRelayBlock")]
    pub included_relay_block: u32,

    #[serde(rename = "paraId")]
    pub para_id: u32,

    pub signature: String,

    /// parachain block hash
    #[serde(rename = "paraHead")]
    pub para_head: String,

    #[serde(rename = "relayParent")]
    pub relay_parent: String,
}
