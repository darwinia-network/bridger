//! Ethereum transaction service
use crate::{pool::EthereumTransaction, result::Result as BridgerResult, service::Service, Config};
use async_trait::async_trait;
use primitives::bytes;
use web3::{
    transports::{http::Http, ws::WebSocket},
    types::{FilterBuilder, H160, H256},
    Transport, Web3,
};

/// Attributes
const SERVICE_NAME: &str = "ethereum";

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService<T: Transport> {
    web3: Web3<T>,
    filter: Vec<FilterBuilder>,
}

impl<T: Transport> EthereumService<T> {
    /// Parse log filter from config
    fn parse_filter(config: &Config) -> BridgerResult<Vec<FilterBuilder>> {
        Ok([&config.eth.contract.bank, &config.eth.contract.issuing]
            .iter()
            .map(|c| {
                FilterBuilder::default()
                    .address(vec![H160::from_slice(&bytes!(c.address.as_str()))])
                    .topics(
                        Some(
                            c.topics
                                .iter()
                                .map(|t| H256::from_slice(&bytes!(t.as_str())))
                                .collect(),
                        ),
                        None,
                        None,
                        None,
                    )
            })
            .collect())
    }

    /// New Ethereum Service with http
    pub async fn new_http(config: &Config) -> BridgerResult<EthereumService<Http>> {
        Ok(EthereumService {
            web3: Web3::new(Http::new(&config.eth.rpc)?),
            filter: Self::parse_filter(&config)?,
        })
    }

    /// New Ethereum Service with websocket
    pub async fn new_ws(config: &Config) -> BridgerResult<EthereumService<WebSocket>> {
        Ok(EthereumService {
            web3: Web3::new(WebSocket::new(&config.eth.rpc).await?),
            filter: Self::parse_filter(&config)?,
        })
    }

    /// Scan ethereum transactions
    pub fn scan(from: u64, to: u64) -> BridgerResult<Vec<EthereumTransaction>> {
        Ok(vec![])
    }
}

#[async_trait]
impl<T: Transport + std::marker::Sync> Service for EthereumService<T> {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&self) -> BridgerResult<()> {
        Ok(())
    }
}
