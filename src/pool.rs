//! Transaction pool
use std::cmp::{Ord, Ordering, PartialOrd};
use web3::types::H256;

/// Ethereum transaction event with hash
#[derive(PartialEq, Eq)]
pub enum EthereumTransactionHash {
    /// Deposit event
    Deposit(H256),
    /// Token event
    Token(H256),
}

/// Reedeemable Ethereum transaction
#[derive(PartialEq, Eq)]
pub struct EthereumTransaction {
    /// Transaction event with hash
    pub hash: EthereumTransactionHash,
    /// Transaction block
    pub block: u64,
}

impl PartialOrd for EthereumTransaction {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        self.block.partial_cmp(&o.block)
    }
}

impl Ord for EthereumTransaction {
    fn cmp(&self, o: &Self) -> Ordering {
        self.block.cmp(&o.block)
    }
}

/// Transaction pool
#[derive(Default)]
pub struct Pool {
    /// Ethereum transactions
    pub ethereum: Vec<EthereumTransaction>,
}
