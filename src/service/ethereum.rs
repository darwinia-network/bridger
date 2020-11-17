//! Ethereum transaction service
use crate::{
    service::{
        redeem::{MsgEthereumTransaction, EthereumTransaction, EthereumTransactionHash},
        relay::MsgBlockNumber,
        MsgStop
    },
    result::{
        Result as BridgerResult, Error
    },
    Config,
    api::Darwinia,
};
use primitives::bytes;

use web3::{
    transports::http::Http,
    types::{BlockNumber, FilterBuilder, H160, H256, U64, SyncState},
    Web3,
};
use actix::prelude::*;
use std::sync::Arc;
use std::time::Duration;

use tokio::fs::File;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
    step: u64,

    relay_service: Recipient<MsgBlockNumber>,
    redeem_service: Recipient<MsgEthereumTransaction>,

    data_dir: PathBuf,
}

impl Actor for EthereumService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(" ✨ SERVICE STARTED: ETHEREUM");
        ctx.run_interval(Duration::from_millis(self.step * 1_000),  |_this, ctx| {
            ctx.notify(MsgScan {});
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!(" 💤 SERVICE STOPPED: ETHEREUM")
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
                        this.relay_service.clone(),
                        this.redeem_service.clone(),
                        this.data_dir.clone(),
                    );
                    f.into_actor(this)
                })
                .map(|r, _, _| {
                    if let Err(e) = r {
                        let err_msg = e.to_string();
                        if err_msg.contains("Scanning ethereum too fast") {
                            warn!("{}", err_msg);
                        } else {
                            error!("{}", err_msg);
                        }
                    }
                }),
        ))
    }
}

impl Handler<MsgStop> for EthereumService {
    type Result = ();

    fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl EthereumService {
    /// New Ethereum Service with http
    pub fn new(
        config: Config,
        web3: Web3<Http>,
        darwinia: Arc<Darwinia>,
        relay_service: Recipient<MsgBlockNumber>,
        redeem_service: Recipient<MsgEthereumTransaction>,
        data_dir: PathBuf,
    ) -> EthereumService
    {
        let step = config.step.ethereum;
        let contracts = EthereumService::parse_contract(&config);
        let filters = EthereumService::parse_filter(&config);
        EthereumService {
            contracts,
            filters,
            web3,
            darwinia,
            step,
            relay_service,
            redeem_service,
            data_dir
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
                  relay_service: Recipient<MsgBlockNumber>,
                  redeem_service: Recipient<MsgEthereumTransaction>,
                  data_dir: PathBuf,
    ) -> BridgerResult<u64> {
        let scan_from = EthereumService::get_scan_from(&darwinia, &web3, &data_dir).await?;

        let latest_block_number = EthereumService::get_latest_block_number(&web3).await?;

        trace!("Heartbeat>>> Scanning on ethereum for new cross-chain transactions from {} to {} ...", scan_from, latest_block_number);

        // 1. Checking start from a right block number
        if scan_from >= latest_block_number {
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

    /// get_latest_block_number
    pub async fn get_latest_block_number(web3: &Web3<Http>) -> BridgerResult<u64> {
        let eth = web3.eth();
        let sync_state = eth.syncing().await?;

        let latest_block_number = match sync_state {
            // TOOD: what the difference between eth_blockNumber and eth_getBlockByNumber("latest", false)
            SyncState::NotSyncing => eth.block_number().await?.as_u64(),
            SyncState::Syncing(info) => info.current_block.as_u64()
        };
        Ok(latest_block_number)
    }

    const ETHEREUM_START_FILE_NAME: &'static str = "ethereum-start";

    /// Get ethereum start from file
    pub async fn get_ethereum_start(data_dir: PathBuf) -> BridgerResult<u64> {
        let mut filepath = data_dir;
        filepath.push(EthereumService::ETHEREUM_START_FILE_NAME);

        // if cache file not exist
        if File::open(&filepath).await.is_err() {
            return Err(Error::Bridger("The ethereum start is not set".to_string()));
        }

        // read start from cache file
        let mut file = File::open(filepath).await?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        match buffer.trim().parse() {
            Ok(start) => Ok(start),
            Err(e) => Err(Error::Bridger(e.to_string()))
        }
    }

    /// Set ethereum start to file
    pub async fn set_ethereum_start(data_dir: PathBuf, value: u64) -> BridgerResult<()> {
        let mut filepath = data_dir;
        filepath.push(EthereumService::ETHEREUM_START_FILE_NAME);
        let mut file = File::create(filepath).await?;
        file.write_all(value.to_string().as_bytes()).await?;
        Ok(())
    }

    async fn get_scan_from(darwinia: &Arc<Darwinia>, web3: &Web3<Http>, data_dir: &PathBuf) -> BridgerResult<u64> {
        if let Ok(block_hash) = darwinia.last_redeemed_ethereum_block().await {
            let block = web3.eth().block(web3::types::BlockId::Hash(block_hash)).await;
            if let Ok(Some(block)) = block {
                return Ok(block.number.unwrap().as_u64());
            }
        }

        match EthereumService::get_ethereum_start(data_dir.clone()).await {
            Ok(ethereum_start) => Ok(ethereum_start),
            Err(e) => {
                return if e.to_string() == "The ethereum start is not set" {
                    Err(Error::Bridger("No ethereum start, run 'bridger set-start --block start' to set one".into()))
                } else {
                    Err(e)
                }
            },
        }
    }
}
