use serde::{Deserialize, Serialize};

use crate::types::MMRProofJson;
use shadow_types::chain::ethereum::receipt::EthereumReceiptJson;

// /// Ethereum ReceiptProofThing Json
// #[derive(Debug, Serialize, Deserialize)]
// pub struct EthereumReceiptJson {
//     /// Ethereum Header
//     pub header: EthereumHeaderJson,
//     /// Ethereum Receipt Proof
//     pub receipt_proof: EthereumReceiptProofJson,
// }
//
// /// Ethereum Receipt Proof Json
// #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
// pub struct EthereumReceiptProofJson {
//     /// Proof index
//     pub index: String,
//     /// Receipt Proof
//     pub proof: String,
//     /// Ethereum Header Hash
//     pub header_hash: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumReceiptWithMMRProof {
    /// Ethereum receipt
    pub receipt: EthereumReceiptJson,
    /// MMR Proof
    pub mmr_proof: MMRProofJson,
}
