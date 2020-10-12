//! Ethereum rpcs
use crate::{chain::eth::EthereumHeaderRPC, result::Result};
use reqwest::Client;
use serde_json::Value;

/// Ethereum JSON rpc response
#[derive(Serialize, Deserialize, Debug)]
pub struct EthHeaderRPCResp {
    jsonrpc: String,
    id: i32,
    /// Header Result of RPC
    pub result: EthereumHeaderRPC,
}

impl EthHeaderRPCResp {
    /// Get `EthHeader` by number
    pub async fn get_by_hash(client: &Client, rpc: &str, block: &str) -> Result<EthHeaderRPCResp> {
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
    pub async fn get(client: &Client, rpc: &str, block: u64) -> Result<EthHeaderRPCResp> {
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
}
