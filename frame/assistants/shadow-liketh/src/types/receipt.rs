use serde::{Deserialize, Serialize};

use crate::types::MMRProofJson;
use shadow_types::chain::ethereum::receipt::EthereumReceiptJson;

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumReceiptWithMMRProof {
    /// Ethereum receipt
    pub receipt: EthereumReceiptJson,
    /// MMR Proof
    pub mmr_proof: MMRProofJson,
}
