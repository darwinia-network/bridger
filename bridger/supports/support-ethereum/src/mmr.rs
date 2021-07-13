use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};

/// Single MMR struct
#[derive(Clone, Decode, Debug, Encode, Default, PartialEq, Eq)]
pub struct MMRRoot {
    /// MMR Root
    pub mmr_root: [u8; 32],
}

/// MMR Root Json
#[derive(Clone, Debug, Decode, Encode, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MMRRootJson {
    /// MMR root string
    pub mmr_root: String,
}

impl From<MMRRootJson> for MMRRoot {
    fn from(that: MMRRootJson) -> Self {
        MMRRoot {
            mmr_root: bridge_primitives::bytes!(that.mmr_root.as_str(), 32),
        }
    }
}

impl From<MMRRoot> for MMRRootJson {
    fn from(that: MMRRoot) -> Self {
        MMRRootJson {
            mmr_root: bridge_primitives::hex!(&that.mmr_root),
        }
    }
}

/// MMR Proof
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

impl From<MMRProofJson> for MMRProof {
    fn from(that: MMRProofJson) -> Self {
        MMRProof {
            member_leaf_index: that.member_leaf_index,
            last_leaf_index: that.last_leaf_index,
            proof: that
                .proof
                .iter()
                .map(|p| bridge_primitives::bytes!(p.as_str(), 32))
                .collect(),
        }
    }
}

impl From<MMRProof> for MMRProofJson {
    fn from(that: MMRProof) -> Self {
        MMRProofJson {
            member_leaf_index: that.member_leaf_index,
            last_leaf_index: that.last_leaf_index,
            proof: that
                .proof
                .iter()
                .map(|p| bridge_primitives::hex!(p))
                .collect(),
        }
    }
}
