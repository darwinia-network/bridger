//! Ethereum receipt rpcs
use bridge_primitives::chain::ethereum::EthReceiptBody;
use reqwest::Client;
use serde_json::Value;

use crate::error::ComponentResult;

/// Ethereum JSON rpc response
#[derive(Serialize, Deserialize, Debug)]
pub struct EthReceiptRPCResp {
    jsonrpc: String,
    id: i32,
    /// Recepit Result of RPC
    pub result: EthReceiptBody,
}

impl EthReceiptRPCResp {
    /// Get `EthReceipt` by txhash
    pub async fn get(
        client: &Client,
        rpc: &str,
        txhash: &str,
    ) -> ComponentResult<EthReceiptRPCResp> {
        let json = format!(
            r#"
			{{
				"jsonrpc":"2.0",
				"method":"eth_getTransactionReceipt",
				"params":["{}"],
				"id": 1
			}}
			"#,
            txhash
        );
        let map: Value = serde_json::from_str(&json)?;
        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }
}
