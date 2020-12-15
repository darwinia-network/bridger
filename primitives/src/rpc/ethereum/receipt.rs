//! Ethereum receipt rpcs
use reqwest::Client;
use serde_json::Value;
use crate::{
    chain::ethereum::{
        EthReceiptBody,
    },
    result::Result
};

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
    pub async fn get(client: &Client, rpc: &str, txhash: &str) -> Result<EthReceiptRPCResp> {
        let map: Value = serde_json::from_str(&format! {
            "{{{}}}", vec![
                r#""jsonrpc":"2.0","#,
                r#""method":"eth_getTransactionReceipt","#,
                &format!(r#""params":["{}"],"#, txhash),
                r#""id": 1"#,
            ].concat(),
        })?;

        Ok(client.post(rpc).json(&map).send().await?.json().await?)
    }
}
