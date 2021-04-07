//! Ethereum transaction service
use crate::{
	service::{
		redeem::{EthereumTransaction, EthereumTransactionHash, MsgEthereumTransaction},
		relay::MsgBlockNumber,
	},
	tools, Settings,
};
use actix::prelude::*;
use std::path::PathBuf;
use web3::{
	transports::http::Http,
	types::{Log, H160, H256},
	Web3,
};
use tokio::time::Duration;
use async_trait::async_trait;

use darwinia::Darwinia;
use ethereum::{Ethereum, TopicsList, LogsHandler, EthereumLikeChain, EthereumLikeChainTracker};


/// Ethereum transaction service
///
/// This service can check and scan darwinia txs in Ethereum
pub struct EthereumService {
	// Ethereum
	data_dir: PathBuf,

	// Darwinia
	darwinia: Darwinia,

	//
	relay_service: Recipient<MsgBlockNumber>,
	redeem_service: Recipient<MsgEthereumTransaction>,
}

#[async_trait]
impl LogsHandler for EthereumService {
    async fn handle(&self, topics_list: Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> ethereum::Result<()> {
		let txs = get_transactions(topics_list, logs).await;
		
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
						"   tx {:?} has already been redeemed.",
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
}

impl EthereumService {
	/// New Ethereum Service with http
	pub fn new(
		config: Settings,
		web3: Web3<Http>,
		data_dir: PathBuf,
		scan_from: u64,
		darwinia: Darwinia,
		relay_service: Recipient<MsgBlockNumber>,
		redeem_service: Recipient<MsgEthereumTransaction>,
	) -> EthereumLikeChainTracker<Ethereum, Self> {
		let contracts = config.get_parsed_contracts();

		let logs_handler = EthereumService {
			data_dir,
			darwinia,
			relay_service,
			redeem_service,
		};

		EthereumLikeChainTracker::new(
			web3,
			EthereumLikeChain::new(
				"Ethereum", 
				TopicsList::new(
					contracts,
					logs_handler,
				), 
				Ethereum::new(scan_from)
			)
		)
	}

}

/// Extract transaction from logs
async fn get_transactions(contracts: Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> Vec<EthereumTransaction> {
	let mut txs = vec![];
	txs.append(
		&mut logs
			.iter()
			.map(|l| {
				let block = l.block_number.unwrap_or_default().low_u64();
				let index = l.transaction_index.unwrap_or_default().low_u64();
				if l.topics.contains(&contracts[1].1[0])
				{
					EthereumTransaction {
						tx_hash: EthereumTransactionHash::Token(
							l.transaction_hash.unwrap_or_default(),
						),
						block_hash: l.block_hash.unwrap_or_default(),
						block,
						index,
					}
				} else if l.topics.contains(&contracts[2].1[0]) {
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
