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
