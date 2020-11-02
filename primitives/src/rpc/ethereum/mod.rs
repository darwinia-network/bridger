//! Ethereum RPC calls
mod block;
mod header;

use crate::{chain::ethereum::EthereumHeader, result::Result, rpc::RPC};
use async_trait::async_trait;
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use self::{block::EthBlockNumberResp, header::EthHeaderRPCResp};

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

    async fn get_header_by_number(&self, block: u64) -> Result<Self::Header> {
        Ok(
            header::EthHeaderRPCResp::get(&self.client, &self.rpc(), block)
                .await?
                .result
                .into(),
        )
    }

    async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header> {
        Ok(
            header::EthHeaderRPCResp::get_by_hash(&self.client, &self.rpc(), block)
                .await?
                .result
                .into(),
        )
    }

    async fn block_number(&self) -> Result<u64> {
        block::block_number(&self.client, &self.rpc()).await
    }
}
