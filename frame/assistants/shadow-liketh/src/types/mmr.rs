use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use serde_hex::{SerHex, StrictPfx};

/// MMR node
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MMRNode {
    /// id
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: u64,
    /// mmr position
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub position: u64,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub hash: [u8; 32],
}

/// MMR Proof Json
#[derive(Clone, Debug, Decode, Encode, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MMRProofJson {
    /// The index of member leaf
    pub member_leaf_index: u64,
    /// The index of of last leaf
    pub last_leaf_index: u64,
    /// The mmr proof of the two leaves above
    pub proof: Vec<[u8; 32]>,
}
