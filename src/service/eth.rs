//! Ethereum transaction service
use crate::{result::Result, service::Service, Config};
use async_trait::async_trait;
use web3::{
    transports::{http::Http, ws::WebSocket},
    Transport, Web3,
};

/// Attributes
const SERVICE_NAME: &str = "ethereum";

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
#[derive(Debug)]
pub struct EthereumService<T: Transport>(Web3<T>);

impl<T: Transport> EthereumService<T> {
    /// New Ethereum Service with http
    pub async fn new_http(url: &str) -> Result<EthereumService<Http>> {
        Ok(EthereumService(Web3::new(Http::new(url)?)))
    }

    /// New Ethereum Service with websocket
    pub async fn new_ws(url: &str) -> Result<EthereumService<WebSocket>> {
        Ok(EthereumService(Web3::new(WebSocket::new(url).await?)))
    }
}

#[async_trait]
impl<T: Transport + std::marker::Sync> Service for EthereumService<T> {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&self, _config: &Config) -> Result<()> {
        Ok(())
    }
}
