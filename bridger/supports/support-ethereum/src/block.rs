use core::fmt::Formatter;
use std::{fmt::Debug, str::FromStr};

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::bytes::to_hex;

use bridge_primitives::{
    array::{Bloom, U256},
    hex,
};

/// Raw EthereumBlock from Ethereum rpc
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumBlockRPC {
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
    transactions_root: String,
    /// Block transactions
    pub transactions: Vec<String>,
    uncles: Vec<String>,
}

impl From<EthereumBlockRPC> for EthereumHeader {
    fn from(that: EthereumBlockRPC) -> Self {
        let seal: Vec<Vec<u8>> = vec![
            rlp::encode(&bytes!(that.mix_hash.as_str())),
            rlp::encode(&bytes!(that.nonce.as_str())),
        ];
        EthereumHeader {
            parent_hash: bytes!(that.parent_hash.as_str(), 32),
            timestamp: u64::from_str_radix(&that.timestamp.as_str()[2..], 16).unwrap_or_default(),
            number: u64::from_str_radix(&that.number.as_str()[2..], 16).unwrap_or_default(),
            author: bytes!(that.miner.as_str(), 20),
            transactions_root: bytes!(that.transactions_root.as_str(), 32),
            uncles_hash: bytes!(that.sha3_uncles.as_str(), 32),
            extra_data: bytes!(that.extra_data.as_str()),
            state_root: bytes!(that.state_root.as_str(), 32),
            receipts_root: bytes!(that.receipts_root.as_str(), 32),
            log_bloom: Bloom(bytes!(that.logs_bloom.as_str(), 256)),
            gas_used: U256::from_str(&that.gas_used[2..]).unwrap_or_default(),
            gas_limit: U256::from_str(&that.gas_limit[2..]).unwrap_or_default(),
            difficulty: U256::from_str(&that.difficulty[2..]).unwrap_or_default(),
            seal,
            hash: match that.hash.is_empty() {
                true => None,
                false => Some(bytes!(that.hash.as_str(), 32)),
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
    log_bloom: Bloom,
    gas_used: U256,
    gas_limit: U256,
    difficulty: U256,
    seal: Vec<Vec<u8>>,
    /// Ethereum header hash
    pub hash: Option<[u8; 32]>,
}

impl std::fmt::Display for EthereumHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msgs = vec![];
        msgs.push(format!(
            "{:>19}{}",
            "parent_hash: ",
            to_hex(&self.parent_hash, false)
        ));
        msgs.push(format!("{:>19}{}", "timestamp: ", &self.timestamp));
        msgs.push(format!("{:>19}{}", "number: ", &self.number));
        msgs.push(format!("{:>19}{}", "author: ", to_hex(&self.author, false)));
        msgs.push(format!(
            "{:>19}{}",
            "transactions_root: ",
            to_hex(&self.transactions_root, false)
        ));
        msgs.push(format!(
            "{:>19}{}",
            "uncles_hash: ",
            to_hex(&self.uncles_hash, false)
        ));
        msgs.push(format!(
            "{:>19}{}",
            "extra_data: ",
            to_hex(&self.extra_data, false)
        ));
        msgs.push(format!(
            "{:>19}{}",
            "state_root: ",
            to_hex(&self.state_root, false)
        ));
        msgs.push(format!(
            "{:>19}{}",
            "receipts_root: ",
            to_hex(&self.receipts_root, false)
        ));
        msgs.push(format!(
            "{:>19}{}",
            "log_bloom: ",
            &self.log_bloom.to_string()
        ));
        msgs.push(format!("{:>19}{}", "gas_used: ", &self.gas_used.as_u128()));
        msgs.push(format!(
            "{:>19}{}",
            "gas_limit: ",
            &self.gas_limit.as_u128()
        ));
        msgs.push(format!(
            "{:>19}{}",
            "difficulty: ",
            &self.difficulty.as_u128()
        ));
        for (i, item) in self.seal.iter().enumerate() {
            if i == 0 {
                msgs.push(format!("{:>19}{}", "seal: ", to_hex(item, false)));
            } else {
                msgs.push(format!("{:>19}{}", "", to_hex(item, false)));
            }
        }
        if let Some(hash) = &self.hash {
            msgs.push(format!("{:>19}{}", "hash: ", to_hex(hash, false)));
        }
        write!(f, "{}", msgs.join("\n"))
    }
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

impl From<EthereumHeader> for EthereumHeaderJson {
    fn from(that: EthereumHeader) -> Self {
        EthereumHeaderJson {
            parent_hash: format!("0x{}", hex!(that.parent_hash.to_vec())),
            timestamp: that.timestamp,
            number: that.number,
            author: format!("0x{}", hex!(that.author.to_vec())),
            transactions_root: format!("0x{}", hex!(that.transactions_root.to_vec())),
            uncles_hash: format!("0x{}", hex!(that.uncles_hash.to_vec())),
            extra_data: format!("0x{}", hex!(that.extra_data.to_vec())),
            state_root: format!("0x{}", hex!(that.state_root.to_vec())),
            receipts_root: format!("0x{}", hex!(that.receipts_root.to_vec())),
            log_bloom: format!("0x{}", hex!(that.log_bloom.0.to_vec())),
            gas_used: that.gas_used.as_u128(),
            gas_limit: that.gas_limit.as_u128(),
            difficulty: that.difficulty.as_u128(),
            seal: that
                .seal
                .iter()
                .map(|s| format!("0x{}", hex!(s.to_vec())))
                .collect(),
            hash: format!("0x{}", hex!(that.hash.unwrap_or_default().to_vec())),
        }
    }
}

impl From<EthereumHeaderJson> for EthereumHeader {
    fn from(that: EthereumHeaderJson) -> Self {
        EthereumHeader {
            parent_hash: bytes!(that.parent_hash.as_str(), 32),
            timestamp: that.timestamp,
            number: that.number,
            author: bytes!(that.author.as_str(), 20),
            transactions_root: bytes!(that.transactions_root.as_str(), 32),
            uncles_hash: bytes!(that.uncles_hash.as_str(), 32),
            extra_data: bytes!(that.extra_data.as_str()),
            state_root: bytes!(that.state_root.as_str(), 32),
            receipts_root: bytes!(that.receipts_root.as_str(), 32),
            log_bloom: Bloom(bytes!(that.log_bloom.as_str(), 256)),
            gas_used: U256::from(that.gas_used),
            gas_limit: U256::from(that.gas_limit),
            difficulty: U256::from(that.difficulty),
            seal: that.seal.iter().map(|s| bytes!(s.as_str())).collect(),
            hash: Some(bytes!(that.hash.as_str(), 32)),
        }
    }
}
