//! Ethereum rpcs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use support_ethereum::block::EthereumBlockRPC;

use crate::error::ComponentEthereumResult;

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
    pub async fn get_by_hash(
        client: &Client,
        rpc: &str,
        block: &str,
    ) -> ComponentEthereumResult<EthBlockRPCResp> {
        let json = format!(
            r#"
			{{
				"jsonrpc":"2.0",
				"method":"eth_getBlockByHash",
				"params":["{}", false],
				"id": 1
			}}
			"#,
            block
        );
        let map: Value = serde_json::from_str(&json)?;
        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }

    /// Get `EthHeader` by number
    pub async fn get(
        client: &Client,
        rpc: &str,
        block: u64,
    ) -> ComponentEthereumResult<EthBlockRPCResp> {
        let json = format!(
            r#"
			{{
				"jsonrpc":"2.0",
				"method":"eth_getBlockByNumber",
				"params":["{:#X}", false],
				"id": 1
			}}
			"#,
            block
        );
        let map: Value = serde_json::from_str(&json)?;
        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }

    /// Get ethereum block number
    pub async fn latest(client: &Client, rpc: &str) -> ComponentEthereumResult<EthBlockRPCResp> {
        let map: Value = serde_json::from_str(
            r#"
			{
				"jsonrpc":"2.0",
				"method":"eth_getBlockByNumber",
				"params":["latest", false],
				"id": 1
			}
			"#,
        )?;
        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }

    /// Get ethereum syncing info
    pub async fn syncing(client: &Client, rpc: &str) -> ComponentEthereumResult<EthSyncingRPCResp> {
        let map: Value = serde_json::from_str(
            r#"
		{
			"jsonrpc":"2.0",
			"method":"eth_syncing",
			"params":[],
			"id": 1
		}
		"#,
        )?;
        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }
}
