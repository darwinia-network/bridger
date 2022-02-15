use serde::{Deserialize, Serialize};

use crate::types::EthashProofJson;

/// Shadow Proposal Response
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EthereumRelayProofsJson {
    /// Ethereum Hash Proof
    pub ethash_proof: Vec<EthashProofJson>,
    /// MMR Proof
    pub mmr_proof: Vec<String>,
}
