use crate::chain::eth::{EthashProof, EthashProofJson, EthereumHeader, EthereumHeaderJson};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Darwinia eth relay header thing
#[derive(Clone, Debug, Decode, Encode, Default, PartialEq, Eq)]
pub struct HeaderStuff {
    eth_header: EthereumHeader,
    ethash_proof: Vec<EthashProof>,
    mmr_root: [u8; 32],
    // mmr_proof: MMRProof,
    mmr_proof: Vec<[u8; 32]>,
}

/// Shadow Proposal Response
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeaderStuffJson {
    header: EthereumHeaderJson,
    ethash_proof: Vec<EthashProofJson>,
    mmr_root: String,
    // mmr_proof: MMRProofJson,
    mmr_proof: Vec<String>,
}

// Shadow Proposal Response
// #[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
// pub struct HeaderStuffOldJson {
//     header: EthereumHeaderJson,
//     ethash_proof: Vec<EthashProofJson>,
//     mmr_root: String,
//     mmr_proof: Vec<String>,
// }

impl Into<HeaderStuff> for HeaderStuffJson {
    fn into(self) -> HeaderStuff {
        HeaderStuff {
            eth_header: self.header.into(),
            ethash_proof: self
                .ethash_proof
                .iter()
                .map(|p| Into::<EthashProof>::into(p.to_owned()))
                .collect(),
            mmr_root: bytes!(self.mmr_root.as_str(), 32),
            // mmr_proof: self.mmr_proof.into(),
            mmr_proof: self
                .mmr_proof
                .iter()
                .map(|p| bytes!(p.as_str(), 32))
                .collect(),
        }
    }
}

// impl Into<HeaderStuff> for HeaderStuffOldJson {
//     fn into(self) -> HeaderStuff {
//         HeaderStuff {
//             eth_header: self.header.into(),
//             ethash_proof: self
//                 .ethash_proof
//                 .iter()
//                 .map(|p| Into::<EthashProof>::into(p.to_owned()))
//                 .collect(),
//             mmr_root: bytes!(self.mmr_root.as_str(), 32),
//             mmr_proof: self
//                 .mmr_proof
//                 .iter()
//                 .map(|p| bytes!(p.as_str(), 32))
//                 .collect(),
//         }
//     }
// }

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
