use web3::{
	transports::http::Http,
	types::{Log, BlockNumber, H160, U64, FilterBuilder, H256, SyncState},
	Web3,
};
use crate::Result;

pub struct EthereumLikeChainClient {
	web3: Web3<Http>,
}

impl EthereumLikeChainClient {
	pub fn new(web3: Web3<Http>) -> EthereumLikeChainClient {
		EthereumLikeChainClient {
			web3,
		}
	}
}

impl EthereumLikeChainClient {
	pub async fn get_logs(&self, contract_address: &H160, topics: &Vec<H256>, from: u64, to: u64) -> Result<Vec<Log>> {
		// build filter
		let filter_builder = FilterBuilder::default()
			.address(vec![contract_address.clone()])
			.topics(Some(topics.clone()), None, None, None);

		let filter = filter_builder.clone()
			.from_block(BlockNumber::Number(U64::from(from)))
			.to_block(BlockNumber::Number(U64::from(to)))
			.build();

		Ok(self.web3.eth().logs(filter).await?)
	}

	pub async fn get_latest_block_number(&self) -> Result<u64> {
		let eth = self.web3.eth();
		let sync_state = eth.syncing().await?;

		let latest_block_number = match sync_state {
			// TOOD: what the difference between eth_blockNumber and eth_getBlockByNumber("latest", false)
			SyncState::NotSyncing => eth.block_number().await?.as_u64(),
			SyncState::Syncing(info) => info.current_block.as_u64(),
		};
		Ok(latest_block_number)
	}
}
