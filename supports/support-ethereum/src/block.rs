use core::fmt::Formatter;
use std::{fmt::Debug, str::FromStr};

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::bytes::to_hex;
use web3::types::{Block, H256};

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
    base_fee_per_gas: Option<String>,
}

impl From<Block<H256>> for EthereumHeader {
    fn from(block: Block<H256>) -> Self {
        let seal = block
            .seal_fields
            .iter()
            .map(|v| v.0.clone())
            .collect::<Vec<Vec<u8>>>();
        Self {
            parent_hash: block.parent_hash.to_fixed_bytes(),
            timestamp: block.timestamp.as_u64(),
            number: block.number.unwrap().as_u64(),
            author: block.author.to_fixed_bytes(),
            transactions_root: block.transactions_root.to_fixed_bytes(),
            uncles_hash: block.uncles_hash.to_fixed_bytes(),
            extra_data: block.extra_data.0,
            state_root: block.state_root.0,
            receipts_root: block.receipts_root.to_fixed_bytes(),
            log_bloom: block.logs_bloom.map(|item| Bloom(item.to_fixed_bytes())),
            gas_used: block.gas_used.as_u128().into(),
            gas_limit: block.gas_limit.as_u128().into(),
            difficulty: block.difficulty.as_u128().into(),
            seal,
            base_fee_per_gas: None,
            hash: block.hash.map(|item| item.to_fixed_bytes()),
        }
    }
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
            log_bloom: Some(Bloom(bytes!(that.logs_bloom.as_str(), 256))),
            gas_used: U256::from_str(&that.gas_used[2..]).unwrap_or_default(),
            gas_limit: U256::from_str(&that.gas_limit[2..]).unwrap_or_default(),
            difficulty: U256::from_str(&that.difficulty[2..]).unwrap_or_default(),
            seal,
            hash: match that.hash.is_empty() {
                true => None,
                false => Some(bytes!(that.hash.as_str(), 32)),
            },
            base_fee_per_gas: that
                .base_fee_per_gas
                .map(|v| U256::from_str(&v[2..]).unwrap_or_default()),
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
    log_bloom: Option<Bloom>,
    gas_used: U256,
    gas_limit: U256,
    difficulty: U256,
    seal: Vec<Vec<u8>>,
    base_fee_per_gas: Option<U256>,
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
        if let Some(log_bloom) = &self.log_bloom {
            msgs.push(format!("{:>19}{}", "log_bloom: ", log_bloom.to_string()));
        }
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
        if let Some(base_fee_per_gas) = &self.base_fee_per_gas {
            msgs.push(format!(
                "{:>19}{}",
                "base_fee_per_gas: ",
                &base_fee_per_gas.as_u128()
            ))
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
    log_bloom: Option<String>,
    gas_used: u128,
    gas_limit: u128,
    difficulty: u128,
    seal: Vec<String>,
    pub base_fee_per_gas: Option<u128>,
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
            log_bloom: that.log_bloom.map(|b| format!("0x{}", hex!(b.0.to_vec()))),
            gas_used: that.gas_used.as_u128(),
            gas_limit: that.gas_limit.as_u128(),
            difficulty: that.difficulty.as_u128(),
            seal: that
                .seal
                .iter()
                .map(|s| format!("0x{}", hex!(s.to_vec())))
                .collect(),
            base_fee_per_gas: that.base_fee_per_gas.map(|v| v.as_u128()),
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
            log_bloom: that
                .log_bloom
                .map(|bloom| Bloom(bytes!(bloom.as_str(), 256))),
            gas_used: U256::from(that.gas_used),
            gas_limit: U256::from(that.gas_limit),
            difficulty: U256::from(that.difficulty),
            seal: that.seal.iter().map(|s| bytes!(s.as_str())).collect(),
            base_fee_per_gas: that.base_fee_per_gas.map(U256::from),
            hash: Some(bytes!(that.hash.as_str(), 32)),
        }
    }
}
