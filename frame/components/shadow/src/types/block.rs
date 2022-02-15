use serde::{Deserialize, Serialize};

/// Darwinia Eth header Json foramt
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Clone)]
pub struct EthereumHeaderJson {
    pub parent_hash: String,
    pub timestamp: u64,
    /// Block Number
    pub number: u64,
    pub author: String,
    pub transactions_root: String,
    pub uncles_hash: String,
    pub extra_data: String,
    pub state_root: String,
    pub receipts_root: String,
    pub log_bloom: String,
    pub gas_used: u128,
    pub gas_limit: u128,
    pub difficulty: u128,
    pub seal: Vec<String>,
    pub base_fee_per_gas: Option<u128>,
    pub hash: String,
}
