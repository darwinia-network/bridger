//! Ethereum transaction service
use crate::{
    memcache::{EthereumTransaction, EthereumTransactionHash, MemCache},
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
use tokio::time;

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
            web3: Web3::new(Http::new(&config.eth.rpc)?),
            step: config.step.ethereum,
        })
    }

    /// New Ethereum Service with websocket
    pub async fn new_ws(config: &Config) -> BridgerResult<EthereumService<WebSocket>> {
        Ok(EthereumService {
            contract: Self::parse_contract(&config),
            filter: Self::parse_filter(&config)?,
            web3: Web3::new(WebSocket::new(&config.eth.rpc).await?),
            step: config.step.ethereum,
        })
    }

    /// Scan ethereum transactions
    pub async fn scan(&self, from: u64, to: u64) -> BridgerResult<Vec<EthereumTransaction>> {
        let mut txs = vec![];
        let eth = self.web3.eth();
        for f in self.filter.iter() {
            let logs = match eth
                .logs(
                    f.clone()
                        .from_block(BlockNumber::Number(U64::from(from)))
                        .to_block(BlockNumber::Number(U64::from(to)))
                        .build(),
                )
                .await
            {
                Ok(logs) => logs,
                Err(e) => {
                    error!("Failed to get logs, due to `{}`", e);

                    continue;
                }
            };

            txs.append(
                &mut logs
                    .iter()
                    .map(|l| {
                        let block = l.block_number.unwrap_or_default().low_u64();
                        let index = l.transaction_index.unwrap_or_default().low_u64();
                        if l.topics.contains(&self.contract.ring)
                            || l.topics.contains(&self.contract.kton)
                        {
                            EthereumTransaction {
                                tx_hash: EthereumTransactionHash::Token(
                                    l.transaction_hash.unwrap_or_default(),
                                ),
                                block_hash: l.block_hash.unwrap_or_default(),
                                block,
                                index,
                            }
                        } else {
                            EthereumTransaction {
                                tx_hash: EthereumTransactionHash::Deposit(
                                    l.transaction_hash.unwrap_or_default(),
                                ),
                                block_hash: l.block_hash.unwrap_or_default(),
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

    async fn run(&mut self, cache: Arc<Mutex<MemCache>>) -> BridgerResult<()> {
        let eth = self.web3.eth();

        loop {
			trace!("Heartbeat>>> Scanning on ethereum for new cross-chain transactions...");
			let block_number = match eth.block_number().await {
				Ok(block_number) => block_number.as_u64(),
				Err(e) => {
					error!("Failed to get ethereum block height, due to `{}`", e);

					time::delay_for(Duration::from_secs(5)).await;

					continue;
				}
			};
            let mut start = u64::MAX;

            if let Ok(cache_cloned) = cache.try_lock() {
                start = cache_cloned.start;
                drop(cache_cloned);
            } else {
                error!("try_lock failed");
            }

            if block_number == start || start == u64::MAX{
                tokio::time::delay_for(Duration::from_secs(self.step)).await;
                continue;
            }

            let mut txs = self.scan(start, block_number).await?;
            if !txs.is_empty() {
                info!("Found {} txs from {} to {}", txs.len(), start, block_number);
                for tx in &txs {
                    trace!("\t{:?}", &tx.tx_hash);
                }
            }

            if let Ok(mut cache_cloned) = cache.try_lock() {
                cache_cloned.txpool.append(&mut txs);
                cache_cloned.start = block_number;
                drop(cache_cloned);
            } else {
                error!("try_lock failed");
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
