//! RPC calls for primitives
#![cfg(feature = "rpc")]

pub mod ethereum;

use crate::result::Result;
use async_trait::async_trait;

/// RPC trait for bridge primitves
#[async_trait]
pub trait Rpc {
	/// Block Header
	type Header;
	/// Transaction Receipt
	type Receipt;
	/// Block
	type Block;

	/// Get block by block hash hex
	async fn get_block_by_hash(&self, block: &str) -> Result<Self::Block>
	where
		Self: Sized;

	/// Get block by block number
	async fn get_block_by_number(&self, block: u64) -> Result<Self::Block>
	where
		Self: Sized;

	/// Get header by block hash hex
	async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header>
	where
		Self: Sized;

	/// Get header by block number
	async fn get_header_by_number(&self, block: u64) -> Result<Self::Header>
	where
		Self: Sized;

	/// Get current highest block
	async fn block_number(&self) -> Result<u64>;

	/// Get receipt
	async fn get_receipt(&self, txhash: &str) -> Result<Self::Receipt>
	where
		Self: Sized;
}

pub use ethereum::EthereumRPC;
