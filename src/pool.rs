//! Transaction pool
use web3::types::H256;

/// Transaction pool
pub struct Pool {
    /// Ethereum transactions
    pub eth: Vec<H256>,
}
