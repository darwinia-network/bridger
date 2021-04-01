//! Ethereum transaction service
use crate::{
	error::{BizError, Result},
	service::{
		redeem::{EthereumTransaction, EthereumTransactionHash, MsgEthereumTransaction},
		relay::MsgBlockNumber,
	},
	tools, Settings,
	config::EthereumContract,
};
use actix::prelude::*;
use std::path::PathBuf;
use web3::{
	transports::http::Http,
	types::{Log, H160, H256},
	Web3,
};
use tokio::time::{delay_for, Duration};

use darwinia::Darwinia;
use ethereum::{Chain, Ethereum, EthereumTracker};
use array_bytes::hex2bytes_unchecked as bytes;

/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService {
	stop: bool,

	// Ethereum
	contracts: Vec<(H160, Vec<H256>)>,
	data_dir: PathBuf,
	tracker: EthereumTracker<ethereum::Ethereum>,

	// Darwinia
	darwinia: Darwinia,

	//
	relay_service: Recipient<MsgBlockNumber>,
	redeem_service: Recipient<MsgEthereumTransaction>,
}

impl EthereumService {
	/// New Ethereum Service with http
	pub fn new(
		config: Settings,
		web3: Web3<Http>,
		scan_from: u64,
		data_dir: PathBuf,
		darwinia: Darwinia,
		relay_service: Recipient<MsgBlockNumber>,
		redeem_service: Recipient<MsgEthereumTransaction>,
	) -> EthereumService {
		let contracts = config.ethereum.contract.clone();
		let contracts = EthereumService::parse_contracts(&contracts);
		let tracker =
			EthereumTracker::new(
				web3.clone(),
				contracts.clone(),
				Ethereum { web3: web3.clone(), from: scan_from },
			);
		EthereumService {
			stop: false,
			contracts,
			data_dir,
			tracker,
			darwinia,
			relay_service,
			redeem_service,
		}
	}

	/// start
	pub async fn start(&mut self) -> Result<()>  {
		info!("✨ SERVICE STARTED: ETHEREUM");

		loop {
			match self.tracker.next().await {
				Err(err) => {
					error!("{:?}", err);
					delay_for(Duration::from_secs(30)).await;
				},
				Ok(logs) => {
					if let Err(err2) = self.handle(logs).await {
						error!("{:?}", err2);
					}
				}
			}

			if self.stop {
				return Err(BizError::Bridger("Force stop".to_string()).into());
			}
		}
	}

	/// stop
	pub fn stop(&mut self) {
		info!("💤 SERVICE STOPPED: ETHEREUM");
		self.stop = true;
	}

	async fn handle(&self, logs: Vec<Log>) -> Result<()> {
		let txs = self.get_transactions(logs).await;
		
		if !txs.is_empty() {
			info!(
				"Found {} txs", txs.len(),
			);
			for tx in &txs {
				trace!("    {:?}", &tx.tx_hash);

				if let Err(e) = self.relay_service.send(MsgBlockNumber(tx.block)).await {
					error!("Send block number to relay service fail: {:?}", e);
				}
			}

			for tx in &txs {
				if self.darwinia.verified(tx.block_hash, tx.index).await? {
					trace!(
						"   {} tx {:?} has already been redeemed.",
						self.tracker.chain.name(),
						tx.enclosed_hash()
					);
					tools::set_cache(
						self.data_dir.clone(),
						tools::LAST_REDEEMED_CACHE_FILE_NAME,
						tx.block,
					)
					.await?;
				} else {
					// delay to wait for possible previous extrinsics
					tokio::time::delay_for(Duration::from_secs(12)).await;
					if let Err(e) = self.redeem_service
						.send(MsgEthereumTransaction { tx: tx.clone() })
						.await
					{
						error!("Send tx to redeem service fail: {:?}", e);
					}
				}
			}
		}

		Ok(())
	}

	/// Extract transaction from logs
	async fn get_transactions(&self, logs: Vec<Log>) -> Vec<EthereumTransaction> {
		let mut txs = vec![];
		txs.append(
			&mut logs
				.iter()
				.map(|l| {
					let block = l.block_number.unwrap_or_default().low_u64();
					let index = l.transaction_index.unwrap_or_default().low_u64();
					if l.topics.contains(&self.contracts[1].1[0])
					{
						EthereumTransaction {
							tx_hash: EthereumTransactionHash::Token(
								l.transaction_hash.unwrap_or_default(),
							),
							block_hash: l.block_hash.unwrap_or_default(),
							block,
							index,
						}
					} else if l.topics.contains(&self.contracts[2].1[0]) {
						EthereumTransaction {
							tx_hash: EthereumTransactionHash::SetAuthorities(
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
		txs
	}

	/// Parse contract addresses and related topics
	fn parse_contracts(contracts: &EthereumContract) -> Vec<(H160, Vec<H256>)> {
		let bank = contracts.bank.clone();
		let issuing = contracts.issuing.clone();
		let relay = contracts.relay.clone();

		let bank_topics = bank.topics.unwrap_or_default()
			.iter()
			.map(|t| H256::from_slice(&bytes(t)))
			.collect();

		let issuing_topics = issuing.topics.unwrap_or_default()
			.iter()
			.map(|t| H256::from_slice(&bytes(t)))
			.collect();

		let relay_topics = relay.topics.unwrap_or_default()
			.iter()
			.map(|t| H256::from_slice(&bytes(t)))
			.collect();

		vec![
			(H160::from_slice(&bytes(bank.address)), bank_topics),
			(H160::from_slice(&bytes(issuing.address)), issuing_topics),
			(H160::from_slice(&bytes(relay.address)), relay_topics),
		]
	}
}
