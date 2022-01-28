use core::fmt::Formatter;
use std::convert::TryFrom;
use std::fmt::Debug;

use codec::{Decode, Encode};
use component_shadow::types::EthereumHeaderJson;
use serde::{Deserialize, Serialize};
use sp_core::bytes::to_hex;
use web3::types::{Block, H256};

use support_primitives::array::{Bloom, U256};

use crate::error::BridgeEthereumError;

impl TryFrom<Block<H256>> for EthereumHeader {
    type Error = BridgeEthereumError;

    fn try_from(block: Block<H256>) -> Result<Self, Self::Error> {
        let seal = block
            .seal_fields
            .iter()
            .map(|v| v.0.clone())
            .collect::<Vec<Vec<u8>>>();
        Ok(Self {
            parent_hash: block.parent_hash.to_fixed_bytes(),
            timestamp: block.timestamp.as_u64(),
            number: block.number.unwrap().as_u64(),
            author: block.author.to_fixed_bytes(),
            transactions_root: block.transactions_root.to_fixed_bytes(),
            uncles_hash: block.uncles_hash.to_fixed_bytes(),
            extra_data: block.extra_data.0,
            state_root: block.state_root.0,
            receipts_root: block.receipts_root.to_fixed_bytes(),
            log_bloom: Bloom(
                block
                    .logs_bloom
                    .ok_or_else(|| {
                        BridgeEthereumError::Other("The `logs_bloom` is required".to_string())
                    })?
                    .to_fixed_bytes(),
            ),
            gas_used: block.gas_used.as_u128().into(),
            gas_limit: block.gas_limit.as_u128().into(),
            difficulty: block.difficulty.as_u128().into(),
            seal,
            base_fee_per_gas: None,
            hash: block.hash.map(|item| item.to_fixed_bytes()),
        })
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
        msgs.push(format!("{:>19}{}", "log_bloom: ", self.log_bloom));
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

impl TryFrom<EthereumHeaderJson> for EthereumHeader {
    type Error = BridgeEthereumError;

    fn try_from(that: EthereumHeaderJson) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_hash: array_bytes::hex2array(that.parent_hash)?,
            timestamp: that.timestamp,
            number: that.number,
            author: array_bytes::hex2array(that.author)?, // bytes!(that.author.as_str(), 20),
            transactions_root: array_bytes::hex2array(that.transactions_root)?,
            uncles_hash: array_bytes::hex2array(that.uncles_hash)?,
            extra_data: array_bytes::hex2bytes(that.extra_data)?, // no length
            state_root: array_bytes::hex2array(that.state_root)?,
            receipts_root: array_bytes::hex2array(that.receipts_root)?,
            log_bloom: Bloom(array_bytes::hex2array(that.log_bloom)?), // Bloom(bytes!(that.log_bloom.as_str(), 256)),
            gas_used: U256::from(that.gas_used),
            gas_limit: U256::from(that.gas_limit),
            difficulty: U256::from(that.difficulty),
            seal: {
                let mut rets = Vec::with_capacity(that.seal.len());
                for item in that.seal {
                    let bytes = array_bytes::hex2bytes(item)?;
                    rets.push(bytes);
                }
                rets
            },
            base_fee_per_gas: that.base_fee_per_gas.map(U256::from),
            hash: Some(array_bytes::hex2array(that.hash)?), //Some(bytes!(that.hash.as_str(), 32)),
        })
    }
}
