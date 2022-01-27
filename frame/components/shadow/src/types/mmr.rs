use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// MMR Root Json
#[derive(Clone, Debug, Decode, Encode, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MMRRootJson {
    /// MMR root string
    pub mmr_root: String,
}
