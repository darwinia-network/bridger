//! Ethereum transaction service
use crate::{
	error::{BizError, Result as BridgerResult},
	service::{
		redeem::{EthereumTransaction, EthereumTransactionHash, MsgEthereumTransaction},
		relay::MsgBlockNumber,
		MsgStop,
	},
	tools, Settings,
};
use primitives::bytes;

use actix::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use web3::{
	transports::http::Http,
	types::{BlockNumber, FilterBuilder, SyncState, H160, H256, U64},
	Web3,
};

use darwinia::Darwinia;

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
	/// relay
	pub relay: H256,
	/// register
	pub register: H256,
	/// lock token on ethereum and redeem from darwinia
	pub lock: H256,
}

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService {
	contracts: ContractAddress,
	filters: [FilterBuilder; 4],
	web3: Web3<Http>,
	darwinia: Darwinia,
	scan_from: u64,
	step: u64,

	relay_service: Recipient<MsgBlockNumber>,
	redeem_service: Recipient<MsgEthereumTransaction>,
	data_dir: PathBuf,
}

impl Actor for EthereumService {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		info!(" ✨ SERVICE STARTED: ETHEREUM");
		ctx.run_interval(Duration::from_millis(self.step * 1_000), |_this, ctx| {
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
						this.darwinia.clone(),
						this.web3.clone(),
						this.contracts.clone(),
						this.filters.clone(),
						this.scan_from,
						this.relay_service.clone(),
						this.redeem_service.clone(),
						this.data_dir.clone(),
					);
					f.into_actor(this)
				})
				.map(|r, this, _| match r {
					Ok(latest_block_number) => this.scan_from = latest_block_number,
					Err(err) => {
						if let Some(e) = err.downcast_ref::<BizError>() {
							trace!("{}", e);
						} else {
							error!("{:?}", err);
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
		config: Settings,
		web3: Web3<Http>,
		darwinia: Darwinia,
		scan_from: u64,
		relay_service: Recipient<MsgBlockNumber>,
		redeem_service: Recipient<MsgEthereumTransaction>,
		data_dir: PathBuf,
	) -> EthereumService {
		let step = config.services.ethereum.step;
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
			data_dir,
		}
	}

	/// Scan ethereum transactions
	async fn do_scan(
		web3: Web3<Http>,
		contracts: ContractAddress,
		filters: [FilterBuilder; 4],
		from: u64,
		to: u64,
	) -> BridgerResult<Vec<EthereumTransaction>> {
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
						if l.topics.contains(&contracts.ring) || l.topics.contains(&contracts.kton)
						{
							EthereumTransaction {
								tx_hash: EthereumTransactionHash::Token(
									l.transaction_hash.unwrap_or_default(),
								),
								block_hash: l.block_hash.unwrap_or_default(),
								block,
								index,
							}
						} else if l.topics.contains(&contracts.relay) {
							EthereumTransaction {
								tx_hash: EthereumTransactionHash::SetAuthorities(
									l.transaction_hash.unwrap_or_default(),
								),
								block_hash: l.block_hash.unwrap_or_default(),
								block,
								index,
							}
						} else if l.topics.contains(&contracts.register) {
							EthereumTransaction {
								tx_hash: EthereumTransactionHash::RegisterErc20Token(
									l.transaction_hash.unwrap_or_default(),
								),
								block_hash: l.block_hash.unwrap_or_default(),
								block,
								index,
							}
						} else if l.topics.contains(&contracts.lock) {
							EthereumTransaction {
								tx_hash: EthereumTransactionHash::RedeemErc20Token(
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

	#[allow(clippy::too_many_arguments)]
	async fn scan(
		darwinia: Darwinia,
		web3: Web3<Http>,
		contracts: ContractAddress,
		filters: [FilterBuilder; 4],
		scan_from: u64,
		relay_service: Recipient<MsgBlockNumber>,
		redeem_service: Recipient<MsgEthereumTransaction>,
		data_dir: PathBuf,
	) -> BridgerResult<u64> {
		let latest_block_number = EthereumService::get_latest_block_number(&web3).await?;

		// 1. Checking start from a right block number
		if scan_from >= latest_block_number {
			return Err(BizError::ScanningEthereumTooFast(scan_from, latest_block_number).into());
		}

		trace!(
			"Heartbeat>>> Scanning on ethereum for new cross-chain transactions from {} to {} ...",
			scan_from,
			latest_block_number
		);

		// 2. Scan tx from ethereum
		let txs =
			EthereumService::do_scan(web3, contracts, filters, scan_from, latest_block_number)
				.await?;
		if !txs.is_empty() {
			info!(
				"Found {} txs from {} to {}",
				txs.len(),
				scan_from,
				latest_block_number
			);
			for tx in &txs {
				trace!("    {:?}", &tx.tx_hash);

				if let Err(e) = relay_service.send(MsgBlockNumber(tx.block)).await {
					error!("Send block number to relay service fail: {:?}", e);
				}
			}

			for tx in &txs {
				if darwinia.verified(tx.block_hash, tx.index).await?
					|| darwinia.verified_issuing(tx.block_hash, tx.index).await?
				{
					trace!(
						"    This ethereum tx {:?} has already been redeemed.",
						tx.enclosed_hash()
					);
					tools::set_cache(
						data_dir.clone(),
						tools::LAST_REDEEMED_CACHE_FILE_NAME,
						tx.block,
					)
					.await?;
				} else {
					// delay to wait for possible previous extrinsics
					tokio::time::delay_for(Duration::from_secs(12)).await;
					if let Err(e) = redeem_service
						.send(MsgEthereumTransaction { tx: tx.clone() })
						.await
					{
						error!("Send tx to redeem service fail: {:?}", e);
					}
				}
			}
		}

		Ok(latest_block_number)
	}

	/// Parse contract addresses
	pub fn parse_contract(config: &Settings) -> ContractAddress {
		let contract = &config.ethereum.contract;
		let bank_topics = contract.bank.topics.clone().unwrap();
		let ring_topics = contract.ring.topics.clone().unwrap();
		let kton_topics = contract.kton.topics.clone().unwrap();
		let relay_topics = contract.relay.topics.clone().unwrap();
		let backing_topics = contract.backing.topics.clone().unwrap();
		ContractAddress {
			bank: H256::from_slice(&bytes!(bank_topics[0].as_str())),
			kton: H256::from_slice(&bytes!(kton_topics[0].as_str())),
			ring: H256::from_slice(&bytes!(ring_topics[0].as_str())),
			relay: H256::from_slice(&bytes!(relay_topics[0].as_str())),
			register: H256::from_slice(&bytes!(backing_topics[0].as_str())),
			lock: H256::from_slice(&bytes!(backing_topics[1].as_str())),
		}
	}

	/// Parse log filter from config
	pub fn parse_filter(config: &Settings) -> [FilterBuilder; 4] {
		let filters = [
			&config.ethereum.contract.bank,
			&config.ethereum.contract.issuing,
			&config.ethereum.contract.backing,
			&config.ethereum.contract.relay,
		]
		.iter()
		.map(|c| {
			let topics = if let Some(topics) = c.topics.clone() {
				topics
					.iter()
					.map(|t| H256::from_slice(&bytes!(t.as_str())))
					.collect()
			} else {
				vec![]
			};
			FilterBuilder::default()
				.address(vec![H160::from_slice(&bytes!(c.address.as_str()))])
				.topics(Some(topics), None, None, None)
		})
		.collect::<Vec<FilterBuilder>>();
		[
			filters[0].clone(),
			filters[1].clone(),
			filters[2].clone(),
			filters[3].clone(),
		]
	}

	/// get_latest_block_number
	pub async fn get_latest_block_number(web3: &Web3<Http>) -> BridgerResult<u64> {
		let eth = web3.eth();
		let sync_state = eth.syncing().await?;

		let latest_block_number = match sync_state {
			// TOOD: what the difference between eth_blockNumber and eth_getBlockByNumber("latest", false)
			SyncState::NotSyncing => eth.block_number().await?.as_u64(),
			SyncState::Syncing(info) => info.current_block.as_u64(),
		};
		Ok(latest_block_number)
	}
}
