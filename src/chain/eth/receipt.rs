//! Ethereum receipt
use crate::{
    bytes,
    chain::eth::{EthereumHeader, EthereumHeaderJson, MMRProof, MMRProofJson},
    hex,
};
use codec::Encode;
use serde::Deserialize;

/// Redeem for
#[derive(Clone, Debug, Encode, PartialEq, Eq)]
pub enum RedeemFor {
    /// Redeem for token
    Token,
    /// Redeem for deopsit
    Deposit,
}

impl Default for RedeemFor {
    fn default() -> Self {
        RedeemFor::Token
    }
}

/// Ethereum Receipt Proof
#[derive(Clone, Debug, Default, Encode, PartialEq, Eq)]
pub struct EthereumReceiptProof {
    /// Proof index
    pub index: u64,
    /// Receipt Proof
    pub proof: Vec<u8>,
    /// Ethereum Header Hash
    pub header_hash: [u8; 32],
}

/// Ethereum Receipt Proof Json
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct EthereumReceiptProofJson {
    /// Proof index
    pub index: String,
    /// Receipt Proof
    pub proof: String,
    /// Ethereum Header Hash
    pub header_hash: String,
}

impl Into<EthereumReceiptProofJson> for EthereumReceiptProof {
    fn into(self) -> EthereumReceiptProofJson {
        EthereumReceiptProofJson {
            index: format!("{:x}", self.index),
            proof: hex!(self.proof),
            header_hash: hex!(self.header_hash.to_vec()),
        }
    }
}

impl Into<EthereumReceiptProof> for EthereumReceiptProofJson {
    fn into(self) -> EthereumReceiptProof {
        let index = if self.index.starts_with("0x") {
            &self.index[2..]
        } else {
            "00"
        };

        let hash = if !self.header_hash.is_empty() {
            bytes!(self.header_hash.as_str(), 32)
        } else {
            [0; 32]
        };

        EthereumReceiptProof {
            index: u64::from_str_radix(index, 16).unwrap_or(0),
            proof: bytes!(self.proof.as_str()),
            header_hash: hash,
        }
    }
}

/// Ethereum ReceiptProofThing
#[derive(Clone, Debug, Default, PartialEq, Eq, Encode)]
pub struct EthereumReceiptProofThing {
    /// Ethereum Header
    pub header: EthereumHeader,
    /// Ethereum Receipt Proof
    pub receipt_proof: EthereumReceiptProof,
    /// MMR Proof
    pub mmr_proof: MMRProof,
}

/// Ethereum ReceiptProofThing Json
#[derive(Debug, Deserialize)]
pub struct EthereumReceiptProofThingJson {
    /// Ethereum Header
    pub header: EthereumHeaderJson,
    /// Ethereum Receipt Proof
    pub receipt_proof: EthereumReceiptProofJson,
    /// MMR Proof
    pub mmr_proof: MMRProofJson,
}

impl Into<EthereumReceiptProofThing> for EthereumReceiptProofThingJson {
    fn into(self) -> EthereumReceiptProofThing {
        EthereumReceiptProofThing {
            header: self.header.into(),
            receipt_proof: self.receipt_proof.into(),
            mmr_proof: self.mmr_proof.into(),
        }
    }
}
