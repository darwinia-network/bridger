//! Transaction pool
use web3::types::H256;

/// Reedeemable Ethereum transactions
pub enum EthereumTransaction {
    /// Deposit event
    Deposit(H256),
    /// Token event
    Token(H256),
}

/// Transaction pool
#[derive(Default)]
pub struct Pool {
    /// Ethereum transactions
    pub eth: Vec<EthereumTransaction>,
}
