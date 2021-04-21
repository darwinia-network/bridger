use crate::{
	Result,
	EthereumLikeChainClient
};
use async_trait::async_trait;
use web3::types::{Log, H160, H256};

#[async_trait]
pub trait LogsHandler {
	async fn handle(&self, client: &EthereumLikeChainClient, topics_list: &Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> Result<()>;
}

pub struct DefaultLogsHandler;

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
	async fn handle(&self, _client: &EthereumLikeChainClient, _topics_list: &Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> Result<()> {
		println!("{:?}", logs);
		Ok(())
	}
}
