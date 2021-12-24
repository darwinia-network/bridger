//! Ethereum Relay Proof
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::error::BridgeEthereumError;
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

impl TryFrom<EthereumRelayProofsJson> for EthereumRelayProofs {
    type Error = BridgeEthereumError;
    fn try_from(that: EthereumRelayProofsJson) -> Result<Self, Self::Error> {
        let mut mmr_proof = Vec::with_capacity(that.mmr_proof.len());
        for item in that.mmr_proof {
            let bytes = array_bytes::hex2array(item)?; // 32
            mmr_proof.push(bytes);
        }
        let mut ethash_proof = Vec::with_capacity(that.ethash_proof.len());
        for item in that.ethash_proof {
            let data = item.try_into()?;
            ethash_proof.push(data);
        }
        Ok(Self {
            ethash_proof,
            mmr_proof,
        })
    }
}
