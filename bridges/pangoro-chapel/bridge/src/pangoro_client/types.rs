use web3::types::{Address, Bytes, H256, U256};

// from contract abi: (parent_hash, state_root, transactions_root, receipts_root, number, timestamp, hash)
pub type Checkpoint = (H256, H256, H256, H256, U256, U256, H256);
pub type TBSCHeader = (
    H256,
    H256,
    Address,
    H256,
    H256,
    H256,
    Bytes,
    U256,
    U256,
    U256,
    U256,
    U256,
    Bytes,
    H256,
    [u8; 8],
);

#[derive(Debug, Clone)]
pub struct BSCHeader {
    pub parent_hash: H256,
    pub uncle_hash: H256,
    pub coinbase: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub log_bloom: Bytes,
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub mix_digest: H256,
    pub nonce: [u8; 8],
}

impl From<TBSCHeader> for BSCHeader {
    fn from(x: TBSCHeader) -> Self {
        BSCHeader {
            parent_hash: x.0,
            uncle_hash: x.1,
            coinbase: x.2,
            state_root: x.3,
            transactions_root: x.4,
            receipts_root: x.5,
            log_bloom: x.6,
            difficulty: x.7,
            number: x.8,
            gas_limit: x.9,
            gas_used: x.10,
            timestamp: x.11,
            extra_data: x.12,
            mix_digest: x.13,
            nonce: x.14,
        }
    }
}

impl From<BSCHeader> for TBSCHeader {
    fn from(x: BSCHeader) -> Self {
        (
            x.parent_hash,
            x.uncle_hash,
            x.coinbase,
            x.state_root,
            x.transactions_root,
            x.receipts_root,
            x.log_bloom,
            x.difficulty,
            x.number,
            x.gas_limit,
            x.gas_used,
            x.timestamp,
            x.extra_data,
            x.mix_digest,
            x.nonce,
        )
    }
}
