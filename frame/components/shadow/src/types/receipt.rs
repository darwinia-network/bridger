use serde::{Deserialize, Serialize};

use crate::types::{EthereumHeaderJson, MMRProofJson};

/// Ethereum ReceiptProofThing Json
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumReceiptProofThingJson {
    /// Ethereum Header
    pub header: EthereumHeaderJson,
    /// Ethereum Receipt Proof
    pub receipt_proof: EthereumReceiptProofJson,
    /// MMR Proof
    pub mmr_proof: MMRProofJson,
}

/// Ethereum Receipt Proof Json
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumReceiptProofJson {
    /// Proof index
    pub index: String,
    /// Receipt Proof
    pub proof: String,
    /// Ethereum Header Hash
    pub header_hash: String,
}
