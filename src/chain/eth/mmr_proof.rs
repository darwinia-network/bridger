use crate::chain::eth::{EthHeader, EthashProof};
use parity_scale_codec::{Decode, Encode};
use serde::Serialize;

/// Darwinia eth relay header thing
#[derive(Decode, Encode, Default)]
pub struct HeaderStuffs {
    eth_header: EthHeader,
    ethash_proof: Vec<EthashProof>,
    mmr_root: [u8; 32],
    mmr_proof: MMRProof,
}

/// MMR Proof Json
#[derive(Decode, Encode, Default)]
pub struct MMRProof {
    /// The index of member leaf
    member_leaf_index: u64,
    /// The index of of last leaf
    last_leaf_index: u64,
    /// The mmrProof of two leaves above
    proof: Vec<[u8; 32]>,
}

/// MMR Proof Json
#[derive(Decode, Encode, Default, Serialize)]
pub struct MMRProofJson {
    /// The index of member leaf
    pub member_leaf_index: u64,
    /// The index of of last leaf
    pub last_leaf_index: u64,
    /// The mmrProof of two leaves above
    pub proof: Vec<String>,
}
