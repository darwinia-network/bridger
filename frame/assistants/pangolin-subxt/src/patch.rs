use crate::api::runtime_types::{darwinia_bridge_ethereum, ethereum_primitives};

/// Ethereum receipt proof
#[derive(Clone, Debug, codec::Encode, codec::Decode)]
pub struct EthereumReceiptProofThing {
    /// Ethereum header
    pub header: ethereum_primitives::header::Header,
    /// Receipt proof
    pub receipt_proof: ethereum_primitives::receipt::ReceiptProof,
    /// MMR proof
    pub mmr_proof: darwinia_bridge_ethereum::MMRProof,
}
