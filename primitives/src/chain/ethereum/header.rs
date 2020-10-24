use crate::{
    array::{H1024, U256},
    hex,
};
use codec::{Decode, Encode};
use std::{fmt::Debug, str::FromStr};

/// Raw EthereumHeader from Ethereum rpc
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumHeaderRPC {
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    gas_used: String,
    /// Ethereum header hash
    pub hash: String,
    logs_bloom: String,
    miner: String,
    mix_hash: String,
    nonce: String,
    number: String,
    /// Parent hash
    pub parent_hash: String,
    receipts_root: String,
    sha3_uncles: String,
    size: String,
    state_root: String,
    timestamp: String,
    total_difficulty: String,
    transactions: Vec<String>,
    transactions_root: String,
    uncles: Vec<String>,
}

impl Into<EthereumHeader> for EthereumHeaderRPC {
    fn into(self) -> EthereumHeader {
        let seal: Vec<Vec<u8>> = vec![
            rlp::encode(&bytes!(self.mix_hash.as_str())),
            rlp::encode(&bytes!(self.nonce.as_str())),
        ];
        EthereumHeader {
            parent_hash: bytes!(self.parent_hash.as_str(), 32),
            timestamp: u64::from_str_radix(&self.timestamp.as_str()[2..], 16).unwrap_or_default(),
            number: u64::from_str_radix(&self.number.as_str()[2..], 16).unwrap_or_default(),
            author: bytes!(self.miner.as_str(), 20),
            transactions_root: bytes!(self.transactions_root.as_str(), 32),
            uncles_hash: bytes!(self.sha3_uncles.as_str(), 32),
            extra_data: bytes!(self.extra_data.as_str()),
            state_root: bytes!(self.state_root.as_str(), 32),
            receipts_root: bytes!(self.receipts_root.as_str(), 32),
            log_bloom: H1024(bytes!(self.logs_bloom.as_str(), 256)),
            gas_used: U256::from_str(&self.gas_used[2..]).unwrap_or_default(),
            gas_limit: U256::from_str(&self.gas_limit[2..]).unwrap_or_default(),
            difficulty: U256::from_str(&self.difficulty[2..]).unwrap_or_default(),
            seal,
            hash: match self.hash.is_empty() {
                true => None,
                false => Some(bytes!(self.hash.as_str(), 32)),
            },
        }
    }
}

/// Darwinia Eth header
#[derive(Clone, Decode, Encode, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EthereumHeader {
    parent_hash: [u8; 32],
    timestamp: u64,
    /// Block number
    pub number: u64,
    author: [u8; 20],
    transactions_root: [u8; 32],
    uncles_hash: [u8; 32],
    extra_data: Vec<u8>,
    state_root: [u8; 32],
    receipts_root: [u8; 32],
    log_bloom: H1024,
    gas_used: U256,
    gas_limit: U256,
    difficulty: U256,
    seal: Vec<Vec<u8>>,
    /// Ethereum header hash
    pub hash: Option<[u8; 32]>,
}

/// Darwinia Eth header Json foramt
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Encode, Clone)]
pub struct EthereumHeaderJson {
    parent_hash: String,
    timestamp: u64,
    /// Block Number
    pub number: u64,
    author: String,
    transactions_root: String,
    uncles_hash: String,
    extra_data: String,
    state_root: String,
    receipts_root: String,
    log_bloom: String,
    gas_used: u128,
    gas_limit: u128,
    difficulty: u128,
    seal: Vec<String>,
    hash: String,
}

impl Into<EthereumHeaderJson> for EthereumHeader {
    fn into(self) -> EthereumHeaderJson {
        EthereumHeaderJson {
            parent_hash: format!("0x{}", hex!(self.parent_hash.to_vec())),
            timestamp: self.timestamp,
            number: self.number,
            author: format!("0x{}", hex!(self.author.to_vec())),
            transactions_root: format!("0x{}", hex!(self.transactions_root.to_vec())),
            uncles_hash: format!("0x{}", hex!(self.uncles_hash.to_vec())),
            extra_data: format!("0x{}", hex!(self.extra_data.to_vec())),
            state_root: format!("0x{}", hex!(self.state_root.to_vec())),
            receipts_root: format!("0x{}", hex!(self.receipts_root.to_vec())),
            log_bloom: format!("0x{}", hex!(self.log_bloom.0.to_vec())),
            gas_used: self.gas_used.as_u128(),
            gas_limit: self.gas_limit.as_u128(),
            difficulty: self.difficulty.as_u128(),
            seal: self
                .seal
                .iter()
                .map(|s| format!("0x{}", hex!(s.to_vec())))
                .collect(),
            hash: format!("0x{}", hex!(self.hash.unwrap_or_default().to_vec())),
        }
    }
}

impl Into<EthereumHeader> for EthereumHeaderJson {
    fn into(self) -> EthereumHeader {
        EthereumHeader {
            parent_hash: bytes!(self.parent_hash.as_str(), 32),
            timestamp: self.timestamp,
            number: self.number,
            author: bytes!(self.author.as_str(), 20),
            transactions_root: bytes!(self.transactions_root.as_str(), 32),
            uncles_hash: bytes!(self.uncles_hash.as_str(), 32),
            extra_data: bytes!(self.extra_data.as_str()),
            state_root: bytes!(self.state_root.as_str(), 32),
            receipts_root: bytes!(self.receipts_root.as_str(), 32),
            log_bloom: H1024(bytes!(self.log_bloom.as_str(), 256)),
            gas_used: U256::from(self.gas_used),
            gas_limit: U256::from(self.gas_limit),
            difficulty: U256::from(self.difficulty),
            seal: self.seal.iter().map(|s| bytes!(s.as_str())).collect(),
            hash: Some(bytes!(self.hash.as_str(), 32)),
        }
    }
}
