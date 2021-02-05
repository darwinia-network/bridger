mod header_mmr;

use jsonrpsee::{
	common::{to_value as to_json_value, Params},
	Client as RpseeClient,
};

use substrate_subxt::sp_core::H256;

use crate::error::Result;

pub use header_mmr::{FormatedMMR, HeaderMMR, HeaderMMRRpc};

/// Rpc interfaces
pub struct Rpc {
	client: RpseeClient,
}

impl Clone for Rpc {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
		}
	}
}

impl Rpc {
	pub fn new(client: RpseeClient) -> Self {
		Self { client }
	}

	pub async fn header_mmr(
		&self,
		block_number_of_member_leaf: u64,
		block_number_of_last_leaf: u64,
		hash: H256,
	) -> Result<Option<HeaderMMR>> {
		let params = Params::Array(vec![
			to_json_value(block_number_of_member_leaf)?,
			to_json_value(block_number_of_last_leaf)?,
		]);
		let result: HeaderMMRRpc = self.client.request("headerMMR_genProof", params).await?;
		let header_mmr: Option<HeaderMMR> = result.into();
		if let Some(mut header_proof) = header_mmr {
			header_proof.block = block_number_of_member_leaf;
			header_proof.hash = hash;
			return Ok(Some(header_proof));
		}
		Ok(None)
	}
}
