use std::sync::atomic::{AtomicUsize, Ordering};

use reqwest::Client;

use support_ethereum::block::{EthereumBlockRPC, EthereumHeader};
use support_ethereum::receipt::EthReceiptBody;

use crate::client::block::EthBlockRPCResp;
use crate::client::rpc::block::EthBlockRPCResp;
use crate::client::rpc::receipt::EthReceiptRPCResp;
use crate::config::EthereumConfig;
use crate::error::{ComponentEthereumError, ComponentEthereumResult};

mod block;
mod receipt;

pub struct EthereumRpcClient {
    /// Reqwest client
    client: Client,
    /// config
    config: EthereumConfig,
    /// atom
    atom: AtomicUsize,
}

impl EthereumRpcClient {
    pub fn new(client: Client, config: EthereumConfig) -> Self {
        let atom = AtomicUsize::new(config.atom);
        Self {
            client,
            config,
            atom,
        }
    }

    /// Generate random RPC
    fn endpoint(&self) -> &str {
        let next = (self.atom.load(Ordering::SeqCst) + 1) % self.config.endpoint.len();
        self.atom.store(next, Ordering::SeqCst);
        &self.config.endpoint[next]
    }
}

/// TODO:
///
/// Verify the result of infura will not response empty header with hash
/// 0x00000...
impl EthereumRpcClient {
    pub async fn get_block_by_hash(
        &self,
        block: &str,
    ) -> ComponentEthereumResult<EthereumBlockRPC> {
        let block = EthBlockRPCResp::get_by_hash(&self.client, self.rpc(), block)
            .await?
            .result;
        Ok(block)
    }

    pub async fn get_block_by_number(
        &self,
        block: u64,
    ) -> ComponentEthereumResult<EthereumBlockRPC> {
        let result = EthBlockRPCResp::get(&self.client, self.rpc(), block).await;
        result.map(|resp| resp.result).map_err(|err| {
            ComponentEthereumError::FailToGetEthereumHeader(format!("{:?}", err), block)
        })
    }

    pub async fn get_header_by_hash(&self, block: &str) -> ComponentEthereumResult<EthereumHeader> {
        Ok(self.get_block_by_hash(block).await?.into())
    }

    pub async fn get_header_by_number(
        &self,
        block: u64,
    ) -> ComponentEthereumResult<EthereumHeader> {
        Ok(self.get_block_by_number(block).await?.into())
    }

    pub async fn get_receipt(&self, txhash: &str) -> ComponentEthereumResult<EthReceiptBody> {
        Ok(EthReceiptRPCResp::get(&self.client, self.rpc(), txhash)
            .await?
            .result)
    }

    pub async fn block_number(&self) -> ComponentEthereumResult<u64> {
        let v: serde_json::Value = EthBlockRPCResp::syncing(&self.client, self.rpc())
            .await?
            .result;
        match v {
            serde_json::Value::Bool(false) => {
                let header: EthereumHeader = EthBlockRPCResp::latest(&self.client, self.rpc())
                    .await?
                    .result
                    .into();

                Ok(header.number)
            }
            serde_json::Value::Object(o) => Ok(u64::from_str_radix(
                o["currentBlock"]
                    .as_str()
                    .unwrap_or_default()
                    .trim_start_matches("0x"),
                16,
            )
            .unwrap_or(0)),
            _ => {
                let header: EthereumHeader = EthBlockRPCResp::latest(&self.client, self.rpc())
                    .await?
                    .result
                    .into();

                Ok(header.number)
            }
        }
    }
}
