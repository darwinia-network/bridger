pub mod darwinia;
pub mod ethereum;
pub mod extrinsics;
pub mod guard;
pub mod redeem;
pub mod relay;
pub mod starter;

use std::hash::{Hash, Hasher};
use web3::types::H256;

/// Ethereum transaction event with hash
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EthereumTransactionHash {
    /// Deposit event
    Deposit(H256),
    /// Token event
    Token(H256),
    /// SetAuthoritiesEvent
    SetAuthorities(H256),
    /// RegisterErc20Token
    RegisterErc20Token(H256),
    /// RedeemErc20Token
    RedeemErc20Token(H256),
}

/// Reedeemable Ethereum transaction
#[derive(Clone, Debug)]
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
            EthereumTransactionHash::SetAuthorities(h) => h,
            EthereumTransactionHash::RegisterErc20Token(h) => h,
            EthereumTransactionHash::RedeemErc20Token(h) => h,
        }
    }
}

impl Hash for EthereumTransaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let txhash = self.enclosed_hash();
        txhash.hash(state);
    }
}

impl PartialEq for EthereumTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block && self.index == other.index
    }
}

impl Eq for EthereumTransaction {}
