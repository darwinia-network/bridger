//! Ethereum transaction service
use crate::{
    service::redeem::{EthereumTransaction, EthereumTransactionHash},
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

#[derive(Clone, Debug)]
struct MsgScan;

impl Message for MsgScan {
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
    darwinia: Arc<Darwinia>,
    scan_from: u64,
    step: u64,

    relay_service: Recipient<MsgBlockNumber>,
    redeem_service: Recipient<MsgEthereumTransaction>,
}

impl Actor for EthereumService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(" 🌟 SERVICE STARTED: ETHEREUM");
        ctx.run_interval(Duration::from_millis(self.step * 1_000),  |_this, ctx| {
            ctx.notify(MsgScan {});
        });
    }
}

impl Handler<MsgScan> for EthereumService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, _msg: MsgScan, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(move |_, this, _| {
                    let f = EthereumService::scan(
                        this.web3.clone(),
                        this.darwinia.clone(),
                        this.contracts.clone(),
                        this.filters.clone(),
                        this.scan_from,
                        this.relay_service.clone(),
                        this.redeem_service.clone(),
                    );
                    f.into_actor(this)
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
use crate::service::relay::MsgBlockNumber;
use crate::service::redeem::MsgEthereumTransaction;
use crate::api::Darwinia;
use std::sync::Arc;

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
    pub fn parse_filter(config: &Config) -> [FilterBuilder; 2] {
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
        [filters[0].clone(), filters[1].clone()]
    }

    /// New Ethereum Service with http
    pub fn new(
        config: Config,
        web3: Web3<Http>,
        darwinia: Arc<Darwinia>,
        relay_service: Recipient<MsgBlockNumber>,
        redeem_service: Recipient<MsgEthereumTransaction>,
    ) -> EthereumService
    {
        let scan_from = config.eth.start;
        let step = config.step.ethereum;
        let contracts = EthereumService::parse_contract(&config);
        let filters = EthereumService::parse_filter(&config);

        EthereumService {
            contracts,
            filters,
            web3,
            darwinia,
            scan_from,
            step,
            relay_service,
            redeem_service,
        }
    }

    /// Scan ethereum transactions
    async fn do_scan(web3: Web3<Http>, contracts: ContractAddress, filters: [FilterBuilder; 2], from: u64, to: u64) -> BridgerResult<Vec<EthereumTransaction>> {
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

    async fn scan(web3: Web3<Http>, darwinia: Arc<Darwinia>,
                  contracts: ContractAddress, filters: [FilterBuilder; 2],
                  scan_from: u64,
                  relay_service: Recipient<MsgBlockNumber>,
                  redeem_service: Recipient<MsgEthereumTransaction>,
    ) -> BridgerResult<u64> {
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
        let txs = EthereumService::do_scan(web3, contracts, filters, scan_from, latest_block_number).await?;
        if !txs.is_empty() {
            info!("Found {} txs from {} to {}", txs.len(), scan_from, latest_block_number);
            for tx in &txs {
                trace!("    {:?}", &tx.tx_hash);

                if let Err(e) = relay_service.send(MsgBlockNumber(tx.block + 1)).await {
                    error!("Send block number to relay service fail: {:?}", e);
                }
            }

            for tx in &txs {
                if darwinia.verified(&tx).await? {
                    warn!("    This ethereum tx {:?} has already been redeemed.", tx.enclosed_hash());
                } else {
                    // delay to wait for possible previous extrinsics
                    tokio::time::delay_for(Duration::from_secs(12)).await;
                    if let Err(e) = redeem_service.send(MsgEthereumTransaction{ tx: tx.clone() }).await {
                        error!("Send tx to redeem service fail: {:?}", e);
                    }
                }
            }
        }

        Ok(latest_block_number)
    }
}
