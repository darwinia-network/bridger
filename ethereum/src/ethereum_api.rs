use crate::Result;
use web3::{
	transports::http::Http,
	types::{Log, BlockNumber, FilterBuilder, H160, H256, U64, SyncState},
	Web3,
};

pub async fn get_logs(web3: &Web3<Http>, contract_address: H160, topics: Vec<H256>, from: u64, to: u64) -> Result<Vec<Log>> {
	// build filter
	let filter_builder = FilterBuilder::default()
		.address(vec![contract_address])
		.topics(Some(topics), None, None, None);

	let filter = filter_builder.clone()
		.from_block(BlockNumber::Number(U64::from(from)))
		.to_block(BlockNumber::Number(U64::from(to)))
		.build();

	Ok(web3.eth().logs(filter).await?)
}

pub async fn get_latest_block_number(web3: &Web3<Http>) -> Result<u64> {
	let eth = web3.eth();
	let sync_state = eth.syncing().await?;

	let latest_block_number = match sync_state {
		// TOOD: what the difference between eth_blockNumber and eth_getBlockByNumber("latest", false)
		SyncState::NotSyncing => eth.block_number().await?.as_u64(),
		SyncState::Syncing(info) => info.current_block.as_u64(),
	};
	Ok(latest_block_number)
}

