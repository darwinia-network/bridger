//! Ethereum rpcs
use crate::{chain::ethereum::EthereumBlockRPC, result::Result};
use reqwest::Client;
use serde_json::Value;

/// Ethereum JSON rpc response
#[derive(Serialize, Deserialize, Debug)]
pub struct EthBlockRPCResp {
	jsonrpc: String,
	id: i32,
	/// Header Result of RPC
	pub result: EthereumBlockRPC,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthSyncingRPCResp {
	jsonrpc: String,
	id: i32,
	/// Header Result of RPC
	pub result: Value,
}

impl EthBlockRPCResp {
	/// Get `EthHeader` by number
	pub async fn get_by_hash(client: &Client, rpc: &str, block: &str) -> Result<EthBlockRPCResp> {
		let map: Value = serde_json::from_str(&format! {
			"{{{}}}", vec![
				r#""jsonrpc":"2.0","#,
				r#""method":"eth_getBlockByHash","#,
				&format!(r#""params":["{}", false],"#, block),
				r#""id": 1"#,
			].concat(),
		})?;

		Ok(client.post(rpc).json(&map).send().await?.json().await?)
	}

	/// Get `EthHeader` by number
	pub async fn get(client: &Client, rpc: &str, block: u64) -> Result<EthBlockRPCResp> {
		let map: Value = serde_json::from_str(&format! {
			"{{{}}}", vec![
				r#""jsonrpc":"2.0","#,
				r#""method":"eth_getBlockByNumber","#,
				&format!(r#""params":["{:#X}", false],"#, block),
				r#""id": 1"#,
			].concat(),
		})?;

		Ok(client.post(rpc).json(&map).send().await?.json().await?)
	}

	/// Get ethereum block number
	pub async fn latest(client: &Client, rpc: &str) -> Result<EthBlockRPCResp> {
		let map: Value = serde_json::from_str(&format! {
			"{{{}}}", vec![
				r#""jsonrpc":"2.0","#,
				r#""method":"eth_getBlockByNumber","#,
				r#""params":["latest", false],"#,
				r#""id": 1"#,
			].concat(),
		})?;

		Ok(client.post(rpc).json(&map).send().await?.json().await?)
	}

	/// Get ethereum syncing info
	pub async fn syncing(client: &Client, rpc: &str) -> Result<EthSyncingRPCResp> {
		let map: Value = serde_json::from_str(&format! {
			"{{{}}}", vec![
				r#""jsonrpc":"2.0","#,
				r#""method":"eth_syncing","#,
				r#""params":[],"#,
				r#""id": 1"#,
			].concat(),
		})?;

		Ok(client.post(rpc).json(&map).send().await?.json().await?)
	}
}
