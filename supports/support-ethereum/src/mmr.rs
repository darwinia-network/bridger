use std::array;

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::error::BridgeEthereumError;

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

impl TryFrom<MMRRootJson> for MMRRoot {
    type Error = BridgeEthereumError;

    fn try_from(that: MMRRootJson) -> Result<Self, Self::Error> {
        Ok(Self {
            mmr_root: array_bytes::hex2array(that.mmr_root.as_str())?, // 32
        })
    }
}

impl TryFrom<MMRRoot> for MMRRootJson {
    type Error = BridgeEthereumError;

    fn try_from(that: MMRRoot) -> Result<Self, Self::Error> {
        Ok(Self {
            mmr_root: array_bytes::bytes2hex("", &that.mmr_root),
        })
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

impl TryFrom<MMRProofJson> for MMRProof {
    type Error = BridgeEthereumError;

    fn try_from(that: MMRProofJson) -> Result<Self, Self::Error> {
        let mut proof = Vec::with_capacity(that.proof.len());
        for item in that.proof {
            let bytes = array_bytes::hex2array(item)?; // 32
            proof.push(bytes);
        }
        Ok(Self {
            member_leaf_index: that.member_leaf_index,
            last_leaf_index: that.last_leaf_index,
            proof,
        })
    }
}

impl TryFrom<MMRProof> for MMRProofJson {
    type Error = BridgeEthereumError;
    fn try_from(that: MMRProof) -> Result<Self, Self::Error> {
        Ok(Self {
            member_leaf_index: that.member_leaf_index,
            last_leaf_index: that.last_leaf_index,
            proof: that
                .proof
                .iter()
                .map(|p| array_bytes::bytes2hex("", p))
                .collect(),
        })
    }
}
