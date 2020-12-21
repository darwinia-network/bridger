//! Ethereum RPC calls
mod block;
mod receipt;

use crate::{chain::ethereum::{EthereumHeader, EthReceiptBody, EthereumBlockRPC}, result::{Result, Error}, rpc::RPC};
use async_trait::async_trait;
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use self::block::EthBlockRPCResp;

/// Ethereum rpc set
pub struct EthereumRPC {
    /// Reqwest client
    pub client: Client,
    /// Rpc host
    pub rpc: Vec<String>,
    /// Counter
    pub atom: AtomicUsize,
}

impl EthereumRPC {
    /// New EthereumRPC
    pub fn new(client: Client, rpc: Vec<String>) -> Self {
        EthereumRPC {
            client,
            rpc,
            atom: AtomicUsize::new(0),
        }
    }

    /// Generate random RPC
    ///
    /// ```
    /// use darwinia_bridge_primitives::rpc::ethereum::EthereumRPC;
    /// use reqwest::Client;
    ///
    /// let rpc = EthereumRPC::new(
    ///   Client::new(),
    ///   vec![
    ///     "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///     "https://mainnet.infura.io/v3/74a9b1b5816b47aa853c23fcc4f2f3b6".to_string(),
    ///   ],
    /// );
    ///
    /// assert_ne!(rpc.rpc(), rpc.rpc());
    /// ```
    pub fn rpc(&self) -> &str {
        let next = (self.atom.load(Ordering::SeqCst) + 1) % self.rpc.len();
        self.atom.store(next, Ordering::SeqCst);
        &self.rpc[next]
    }
}

/// TODO:
///
/// Verify the result of infura will not response empty header with hash
/// 0x00000...
#[async_trait]
impl RPC for EthereumRPC {
    type Header = EthereumHeader;
    type Receipt = EthReceiptBody;
    type Block = EthereumBlockRPC;

    async fn get_block_by_hash(&self, block: &str) -> Result<Self::Block> {
        Ok(
            EthBlockRPCResp::get_by_hash(&self.client, &self.rpc(), block)
                .await?
                .result
        )
    }

    async fn get_block_by_number(&self, block: u64) -> Result<Self::Block> {
        let result = EthBlockRPCResp::get(&self.client, &self.rpc(), block)
            .await;
        result
            .map(|resp| resp.result)
            .map_err(|err|
                Error::FailToGetEthereumHeader(format!("{:?}", err), block)
            )
    }

    async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header> {
        Ok (
            self.get_block_by_hash(block)
                .await?
                .into()
        )
    }

    async fn get_header_by_number(&self, block: u64) -> Result<Self::Header> {
        Ok (
            self.get_block_by_number(block)
                .await?
                .into()
        )
    }

    async fn get_receipt(&self, txhash: &str) -> Result<Self::Receipt> {
        Ok(
            receipt::EthReceiptRPCResp::get(&self.client, &self.rpc(), txhash)
                .await?
                .result,
        )
    }

    async fn block_number(&self) -> Result<u64> {
        let v: serde_json::Value = EthBlockRPCResp::syncing(&self.client, &self.rpc()).await?.result;
        match v {
            serde_json::Value::Bool(false) => {
                let header: Self::Header  = EthBlockRPCResp::latest(&self.client, &self.rpc())
                    .await?
                    .result
                    .into();

                Ok(
                    header.number
                )
            },
            serde_json::Value::Object(o) => {
                Ok(
                    u64::from_str_radix(o["currentBlock"].as_str().unwrap_or_default().trim_start_matches("0x"), 16).unwrap_or(0)
                )
            },
            _ => {
                let header: Self::Header  = EthBlockRPCResp::latest(&self.client, &self.rpc())
                    .await?
                    .result
                    .into();

                Ok(
                    header.number
                )
            }
        }
    }
}
