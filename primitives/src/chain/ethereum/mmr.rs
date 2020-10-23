use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// MMR Proof Json
#[derive(Clone, Decode, Debug, Encode, Default, PartialEq, Eq)]
pub struct MMRProof {
    /// The index of member leaf
    pub member_leaf_index: u64,
    /// The index of of last leaf
    pub last_leaf_index: u64,
    /// The mmrProof of two leaves above
    pub proof: Vec<[u8; 32]>,
}

/// MMR Proof Json
#[derive(Clone, Debug, Decode, Encode, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MMRProofJson {
    /// The index of member leaf
    pub member_leaf_index: u64,
    /// The index of of last leaf
    pub last_leaf_index: u64,
    /// The mmr proof of the two leaves above
    pub proof: Vec<String>,
}

impl Into<MMRProof> for MMRProofJson {
    fn into(self) -> MMRProof {
        MMRProof {
            member_leaf_index: self.member_leaf_index,
            last_leaf_index: self.last_leaf_index,
            proof: self.proof.iter().map(|p| bytes!(p.as_str(), 32)).collect(),
        }
    }
}

impl Into<MMRProofJson> for MMRProof {
    fn into(self) -> MMRProofJson {
        MMRProofJson {
            member_leaf_index: self.member_leaf_index,
            last_leaf_index: self.last_leaf_index,
            proof: self.proof.iter().map(|p| hex!(p)).collect(),
        }
    }
}
