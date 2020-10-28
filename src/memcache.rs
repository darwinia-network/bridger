//! Transaction pool
use std::cmp::{Ord, Ordering, PartialOrd};
use web3::types::H256;

/// Ethereum transaction event with hash
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EthereumTransactionHash {
    /// Deposit event
    Deposit(H256),
    /// Token event
    Token(H256),
}

/// Reedeemable Ethereum transaction
#[derive(PartialEq, Eq, Clone)]
pub struct EthereumTransaction {
    /// Transaction hash for the event
    pub tx_hash: EthereumTransactionHash,
    /// Block Hash for the event
    pub block_hash: H256,
    /// Transaction block
    pub block: u64,
    /// Transaction index
    pub index: u64,
}

impl EthereumTransaction {
    /// Get the hash
    pub fn enclosed_hash(&self) -> H256 {
        match self.tx_hash {
            EthereumTransactionHash::Token(h) => h,
            EthereumTransactionHash::Deposit(h) => h,
        }
    }
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

/// Memory Cache
#[derive(Default)]
pub struct MemCache {
    /// Start ethereum block to scan with
    pub start: u64,
    /// Ethereum transactions
    pub txpool: Vec<EthereumTransaction>,
}

impl MemCache {
    /// New an instance of the MemCache
    pub fn new(start: u64) -> Self {
        MemCache {
            start,
            txpool: Vec::new()
        }
    }
}