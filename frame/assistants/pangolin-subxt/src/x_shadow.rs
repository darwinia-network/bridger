// use web3::types::{Block, H256};
//
// use crate::BridgeEthereumError;
//
// impl TryFrom<Block<H256>> for EthereumHeader {
//     type Error = BridgeEthereumError;
//
//     fn try_from(block: Block<H256>) -> Result<Self, Self::Error> {
//         let seal = block
//             .seal_fields
//             .iter()
//             .map(|v| v.0.clone())
//             .collect::<Vec<Vec<u8>>>();
//         Ok(Self {
//             parent_hash: block.parent_hash.to_fixed_bytes(),
//             timestamp: block.timestamp.as_u64(),
//             number: block.number.unwrap().as_u64(),
//             author: block.author.to_fixed_bytes(),
//             transactions_root: block.transactions_root.to_fixed_bytes(),
//             uncles_hash: block.uncles_hash.to_fixed_bytes(),
//             extra_data: block.extra_data.0,
//             state_root: block.state_root.0,
//             receipts_root: block.receipts_root.to_fixed_bytes(),
//             log_bloom: Bloom(
//                 block
//                     .logs_bloom
//                     .ok_or_else(|| {
//                         BridgeEthereumError::Other("The `logs_bloom` is required".to_string())
//                     })?
//                     .to_fixed_bytes(),
//             ),
//             gas_used: block.gas_used.as_u128().into(),
//             gas_limit: block.gas_limit.as_u128().into(),
//             difficulty: block.difficulty.as_u128().into(),
//             seal,
//             base_fee_per_gas: None,
//             hash: block.hash.map(|item| item.to_fixed_bytes()),
//         })
//     }
// }

use crate::api::runtime_types;
use component_shadow::types::HeaderParcel;

use crate::api::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::api::runtime_types::ethereum_primitives;
use crate::ConvertTypeError;

impl TryFrom<HeaderParcel> for EthereumRelayHeaderParcel {
    type Error = ConvertTypeError;

    fn try_from(value: HeaderParcel) -> Result<Self, Self::Error> {
        Ok(Self {
            header: value.header.try_into()?,
            parent_mmr_root: Default::default(),
        })
    }
}

impl TryFrom<web3::types::Block<web3::types::H256>> for ethereum_primitives::header::Header {
    type Error = ConvertTypeError;

    fn try_from(value: web3::types::Block<web3::types::H256>) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_hash: subxt::sp_core::H256(value.parent_hash.to_fixed_bytes()),
            timestamp: value.timestamp.as_u64(),
            number: value
                .number
                .ok_or_else(|| Self::Error::Other(format!("Missing ethereum block number")))?
                .as_u64(),
            author: runtime_types::primitive_types::H160(value.author.to_fixed_bytes()),
            transactions_root: subxt::sp_core::H256(value.transactions_root.to_fixed_bytes()),
            uncles_hash: subxt::sp_core::H256(value.uncles_hash.to_fixed_bytes()),
            extra_data: value.extra_data.0,
            state_root: subxt::sp_core::H256(value.state_root.to_fixed_bytes()),
            receipts_root: subxt::sp_core::H256(value.receipts_root.to_fixed_bytes()),
            log_bloom: runtime_types::ethbloom::Bloom(
                value
                    .logs_bloom
                    .ok_or_else(|| Self::Error::Other("The `logs_bloom` is required".to_string()))?
                    .to_fixed_bytes(),
            ),
            gas_used: runtime_types::primitive_types::U256(value.gas_used.0),
            gas_limit: runtime_types::primitive_types::U256(value.gas_limit.0),
            difficulty: runtime_types::primitive_types::U256(value.difficulty.0),
            seal: value
                .seal_fields
                .iter()
                .map(|v| v.0.clone())
                .collect::<Vec<Vec<u8>>>(),
            base_fee_per_gas: None,
            hash: value
                .hash
                .map(|item| subxt::sp_core::H256(item.to_fixed_bytes())),
        })
    }
}
