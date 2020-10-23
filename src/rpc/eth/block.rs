use crate::result::Result;
use reqwest::Client;
use serde_json::Value;
use std::fmt::Debug;

/// Ethereum current block rpc resp
#[derive(Serialize, Deserialize, Debug)]
pub struct EthBlockNumberResp {
    jsonrpc: String,
    id: i32,
    /// Block number result of RPC
    pub result: String,
}

/// Get ethereum block number
pub async fn block_number(client: &Client, rpc: &str) -> Result<u64> {
    let map: Value = serde_json::from_str(&format! {
        "{{{}}}", vec![
            r#""jsonrpc":"2.0","#,
            r#""method":"eth_blockNumber","#,
            r#""params":[],"#,
            r#""id": 1"#,
        ].concat(),
    })?;

    Ok(u64::from_str_radix(
        &client
            .post(rpc)
            .json(&map)
            .send()
            .await?
            .json::<EthBlockNumberResp>()
            .await?
            .result
            .trim_start_matches("0x"),
        16,
    )
    .unwrap_or(0))
}
