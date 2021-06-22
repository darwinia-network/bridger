use bridge_config::component::EthereumRpcConfig;
use bridge_primitives::chain::ethereum::{EthReceiptBody, EthereumBlockRPC, EthereumHeader};
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::{ComponentError, ComponentResult};
use crate::ethereum_rpc::block::EthBlockRPCResp;
use crate::ethereum_rpc::receipt::EthReceiptRPCResp;

/// Ethereum rpc set
pub struct EthereumRpc {
    /// Reqwest client
    client: Client,
    /// config
    config: EthereumRpcConfig,
    /// atom
    atom: AtomicUsize,
}

impl EthereumRpc {
    pub fn new(client: Client, config: EthereumRpcConfig) -> Self {
        let atom = AtomicUsize::new(config.atom);
        Self {
            client,
            config,
            atom,
        }
    }
    /// Generate random RPC
    pub fn rpc(&self) -> &str {
        let next = (self.atom.load(Ordering::SeqCst) + 1) % self.config.rpc.len();
        self.atom.store(next, Ordering::SeqCst);
        &self.config.rpc[next]
    }
}

/// TODO:
///
/// Verify the result of infura will not response empty header with hash
/// 0x00000...
// #[async_trait]
impl EthereumRpc {
    // type Header = EthereumHeader;
    // type Receipt = EthReceiptBody;
    // type Block = EthereumBlockRPC;

    pub async fn get_block_by_hash(&self, block: &str) -> ComponentResult<EthereumBlockRPC> {
        Ok(
            EthBlockRPCResp::get_by_hash(&self.client, &self.rpc(), block)
                .await?
                .result,
        )
    }

    pub async fn get_block_by_number(&self, block: u64) -> ComponentResult<EthereumBlockRPC> {
        let result = EthBlockRPCResp::get(&self.client, &self.rpc(), block).await;
        result
            .map(|resp| resp.result)
            .map_err(|err| ComponentError::FailToGetEthereumHeader(format!("{:?}", err), block))
    }

    pub async fn get_header_by_hash(&self, block: &str) -> ComponentResult<EthereumHeader> {
        Ok(self.get_block_by_hash(block).await?.into())
    }

    pub async fn get_header_by_number(&self, block: u64) -> ComponentResult<EthereumHeader> {
        Ok(self.get_block_by_number(block).await?.into())
    }

    pub async fn get_receipt(&self, txhash: &str) -> ComponentResult<EthReceiptBody> {
        Ok(EthReceiptRPCResp::get(&self.client, &self.rpc(), txhash)
            .await?
            .result)
    }

    pub async fn block_number(&self) -> ComponentResult<u64> {
        let v: serde_json::Value = EthBlockRPCResp::syncing(&self.client, &self.rpc())
            .await?
            .result;
        match v {
            serde_json::Value::Bool(false) => {
                let header: EthereumHeader = EthBlockRPCResp::latest(&self.client, &self.rpc())
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
                let header: EthereumHeader = EthBlockRPCResp::latest(&self.client, &self.rpc())
                    .await?
                    .result
                    .into();

                Ok(header.number)
            }
        }
    }
}
