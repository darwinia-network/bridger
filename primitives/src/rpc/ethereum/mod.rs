//! Ethereum RPC calls
mod block;
mod header;

use crate::{chain::ethereum::EthereumHeader, result::Result, rpc::RPC};
use async_trait::async_trait;
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use self::{block::EthBlockNumberResp, header::EthHeaderRPCResp};

/// Ethereum rpc set
pub struct EthereumRPC<'r> {
    /// Reqwest client
    pub client: &'r Client,
    /// Rpc host
    pub rpc: Vec<&'r str>,
    /// Counter
    pub atom: AtomicUsize,
}

impl<'r> EthereumRPC<'r> {
    /// New EthereumRPC
    pub fn new(client: &'r Client, rpc: Vec<&'r str>) -> Self {
        EthereumRPC {
            client,
            rpc,
            atom: AtomicUsize::new(0),
        }
    }

    /// Generate random RPC
    pub fn rpc(&self) -> &'r str {
        self.atom.fetch_add(1, Ordering::SeqCst);
        self.rpc[self.atom.load(Ordering::SeqCst) % self.rpc.len()]
    }
}

#[async_trait]
impl<'r> RPC for EthereumRPC<'r> {
    type Header = EthereumHeader;

    async fn get_header_by_number(&self, block: u64) -> Result<Self::Header> {
        Ok(
            header::EthHeaderRPCResp::get(self.client, self.rpc(), block)
                .await?
                .result
                .into(),
        )
    }

    async fn get_header_by_hash(&self, block: &str) -> Result<Self::Header> {
        Ok(
            header::EthHeaderRPCResp::get_by_hash(self.client, self.rpc(), block)
                .await?
                .result
                .into(),
        )
    }

    async fn block_number(&self) -> Result<u64> {
        block::block_number(self.client, self.rpc()).await
    }
}
