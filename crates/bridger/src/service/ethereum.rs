//! Ethereum transaction service
use crate::{
    pool::{EthereumTransaction, EthereumTransactionHash, Pool},
    result::Result as BridgerResult,
    service::Service,
    Config,
};
use async_trait::async_trait;
use primitives::bytes;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use web3::{
    transports::{http::Http, ws::WebSocket},
    types::{BlockNumber, FilterBuilder, H160, H256, U64},
    Transport, Web3,
};

/// Attributes
const SERVICE_NAME: &str = "ETHEREUM";

/// Darwinia contract addresses
pub struct ContractAddress {
    ring: H256,
    kton: H256,
    #[allow(dead_code)]
    bank: H256,
}

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService<T: Transport> {
    contract: ContractAddress,
    filter: [FilterBuilder; 2],
    web3: Web3<T>,
    start: u64,
    step: u64,
}

impl<T: Transport> EthereumService<T> {
    /// Parse contract addresses
    fn parse_contract(config: &Config) -> ContractAddress {
        let contract = &config.eth.contract;
        ContractAddress {
            bank: H256::from_slice(&bytes!(contract.bank.topics[0].as_str())),
            kton: H256::from_slice(&bytes!(contract.kton.topics[0].as_str())),
            ring: H256::from_slice(&bytes!(contract.ring.topics[0].as_str())),
        }
    }

    /// Parse log filter from config
    fn parse_filter(config: &Config) -> BridgerResult<[FilterBuilder; 2]> {
        let filters = [&config.eth.contract.bank, &config.eth.contract.issuing]
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
            .collect::<Vec<FilterBuilder>>();
        Ok([filters[0].clone(), filters[1].clone()])
    }

    /// New Ethereum Service with http
    pub fn new_http(config: &Config) -> BridgerResult<EthereumService<Http>> {
        Ok(EthereumService {
            contract: Self::parse_contract(&config),
            filter: Self::parse_filter(&config)?,
            start: config.eth.start,
            web3: Web3::new(Http::new(&config.eth.rpc)?),
            step: config.step.ethereum,
        })
    }

    /// New Ethereum Service with websocket
    pub async fn new_ws(config: &Config) -> BridgerResult<EthereumService<WebSocket>> {
        Ok(EthereumService {
            contract: Self::parse_contract(&config),
            filter: Self::parse_filter(&config)?,
            start: config.eth.start,
            web3: Web3::new(WebSocket::new(&config.eth.rpc).await?),
            step: config.step.ethereum,
        })
    }

    /// Scan ethereum transactions
    pub async fn scan(&self, from: u64, to: u64) -> BridgerResult<Vec<EthereumTransaction>> {
        let mut txs = vec![];
        let eth = self.web3.eth();
        for f in self.filter.iter() {
            txs.append(
                &mut eth
                    .logs(
                        f.clone()
                            .from_block(BlockNumber::Number(U64::from(from)))
                            .to_block(BlockNumber::Number(U64::from(to)))
                            .build(),
                    )
                    .await?
                    .iter()
                    .map(|l| {
                        let block = l.block_number.unwrap_or_default().low_u64();
                        let index = l.transaction_index.unwrap_or_default().low_u64();
                        if l.topics.contains(&self.contract.ring)
                            || l.topics.contains(&self.contract.kton)
                        {
                            EthereumTransaction {
                                hash: EthereumTransactionHash::Token(
                                    l.transaction_hash.unwrap_or_default(),
                                ),
                                block,
                                index,
                            }
                        } else {
                            EthereumTransaction {
                                hash: EthereumTransactionHash::Deposit(
                                    l.transaction_hash.unwrap_or_default(),
                                ),
                                block,
                                index,
                            }
                        }
                    })
                    .collect::<Vec<EthereumTransaction>>(),
            );
        }
        Ok(txs)
    }
}
#[async_trait(?Send)]
impl<T: Transport + std::marker::Sync> Service for EthereumService<T> {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, pool: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        let eth = self.web3.eth();
        let mut block_number: u64;
        let mut start = self.start;

        loop {
            if let Ok(mut pool_cloned) = pool.try_lock() {
                trace!("Looking for darwinia ethereum transactions...");
                block_number = eth.block_number().await?.as_u64();
                if block_number == start {
                    tokio::time::delay_for(Duration::from_secs(self.step)).await;
                    continue;
                }

                let mut txs = self.scan(start, block_number).await?;
                if !txs.is_empty() {
                    info!("Found {} txs from {} to {}", txs.len(), start, block_number);
                    for tx in &txs {
                        trace!("\t{:?}", &tx.hash);
                    }
                }

                pool_cloned.ethereum.append(&mut txs);
                start = block_number;
                drop(pool_cloned);
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
