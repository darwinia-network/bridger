use serde::{Deserialize, Serialize};

use serde_hex::SerHexSeq;
use serde_hex::StrictPfx;

/// MMR position
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MMRPosition {
    /// id
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: u64,
    /// position
    pub position: String,
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    pub hash: Vec<u8>,
}
