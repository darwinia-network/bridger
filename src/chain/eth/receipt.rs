//! Ethereum receipt
use codec::Encode;

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
