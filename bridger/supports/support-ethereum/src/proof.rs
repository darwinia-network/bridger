//! Ethereum Relay Proof
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::ethash::{EthashProof, EthashProofJson};

/// Darwinia eth relay header thing
#[derive(Clone, Debug, Decode, Encode, Default, PartialEq, Eq)]
pub struct EthereumRelayProofs {
    /// Ethereum Hash Proof
    pub ethash_proof: Vec<EthashProof>,
    /// MMR Proof
    pub mmr_proof: Vec<[u8; 32]>,
}

/// Shadow Proposal Response
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EthereumRelayProofsJson {
    /// Ethereum Hash Proof
    pub ethash_proof: Vec<EthashProofJson>,
    /// MMR Proof
    pub mmr_proof: Vec<String>,
}

impl From<EthereumRelayProofsJson> for EthereumRelayProofs {
    fn from(that: EthereumRelayProofsJson) -> Self {
        EthereumRelayProofs {
            ethash_proof: that
                .ethash_proof
                .iter()
                .map(|p| Into::<EthashProof>::into(p.to_owned()))
                .collect(),
            mmr_proof: that
                .mmr_proof
                .iter()
                .map(|p| bridge_primitives::bytes!(p.as_str(), 32))
                .collect(),
        }
    }
}
