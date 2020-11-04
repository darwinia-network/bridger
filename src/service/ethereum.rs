//! Ethereum transaction service
use crate::{
    memcache::{EthereumTransaction, EthereumTransactionHash},
    result::Result as BridgerResult,
    Config,
};
use std::{
    time::Duration,
};
use web3::{
    transports::http::Http,
    types::{BlockNumber, FilterBuilder, H160, H256, U64},
    Web3,
};

use actix::prelude::*;
use crate::result::Error;

/// message 'Start'
#[derive(Clone, Debug)]
pub struct MsgStart;

impl Message for MsgStart {
    type Result = ();
}

/// Darwinia contract addresses
#[derive(Clone, Debug)]
pub struct ContractAddress {
    /// r
    pub ring: H256,
    /// k
    pub kton: H256,
    /// b
    #[allow(dead_code)]
    pub bank: H256,
}

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService {
    contracts: ContractAddress,
    filters: [FilterBuilder; 2],
    web3: Web3<Http>,
    scan_from: u64,
    step: u64,
}

impl Actor for EthereumService {
    type Context = Context<Self>;
}

impl Handler<MsgStart> for EthereumService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: MsgStart, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(move |_, this, _| {
                    let f = EthereumService::run(this.web3.clone(), this.contracts.clone(), this.filters.clone(), this.scan_from);
                    f.into_actor(this)
                })
                .then(|r, this, ctx| {
                    ctx.notify_later(msg, Duration::from_millis(this.step * 1000));
                    async {r}.into_actor(this)
                })
                .map(|r, this, _| {
                    if let Ok(latest_block_number) = r {
                        this.scan_from = latest_block_number
                    }
                }),
        ))
    }
}
use primitives::bytes;
impl EthereumService {
    /// Parse contract addresses
    pub fn parse_contract(config: &Config) -> ContractAddress {
        let contract = &config.eth.contract;
        let a = contract.bank.topics[0].as_str();
        ContractAddress {
            bank: H256::from_slice(&bytes!(a)),
            kton: H256::from_slice(&bytes!(contract.kton.topics[0].as_str())),
            ring: H256::from_slice(&bytes!(contract.ring.topics[0].as_str())),
        }
    }

    /// Parse log filter from config
    pub fn parse_filter(config: &Config) -> BridgerResult<[FilterBuilder; 2]> {
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
    pub fn new(
        web3: Web3<Http>,
        contracts: ContractAddress, filters: [FilterBuilder; 2],
        scan_from: u64, step: u64) -> EthereumService
    {
        EthereumService {
            contracts,
            filters,
            web3,
            scan_from,
            step,
        }
    }

    /// Scan ethereum transactions
    pub async fn scan(web3: Web3<Http>, contracts: ContractAddress, filters: [FilterBuilder; 2], from: u64, to: u64) -> BridgerResult<Vec<EthereumTransaction>> {
        let mut txs = vec![];
        let eth = web3.eth();
        for f in filters.iter() {
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
                        if l.topics.contains(&contracts.ring)
                            || l.topics.contains(&contracts.kton)
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

    async fn run(web3: Web3<Http>, contracts: ContractAddress, filters: [FilterBuilder; 2], scan_from: u64) -> BridgerResult<u64> {
        trace!("Heartbeat>>> Scanning on ethereum for new cross-chain transactions from {}...", scan_from);

        let eth = web3.eth();
        let latest_block_number = eth.block_number().await?.as_u64();

        // 1. Checking start from a right block number
        if scan_from == latest_block_number {
            let msg = format!("Scanning ethereum too fast: {}", scan_from);
            return Err(Error::Bridger(msg));
        }

        if scan_from == u64::MAX {
            let msg = "Scanning ethereum to u64::MAX".to_string();
            return Err(Error::Bridger(msg));
        }

        // 2. Scan tx from ethereum
        let txs = EthereumService::scan(web3, contracts, filters, scan_from, latest_block_number).await?;
        if !txs.is_empty() {
            info!("Found {} txs from {} to {}", txs.len(), scan_from, latest_block_number);
            for tx in &txs {
                trace!("\t{:?}", &tx.tx_hash);

                // TODO: send msg to relay and redeem service
            }
        }

        Ok(latest_block_number)
    }
}
