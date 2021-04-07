use web3::{
	transports::http::Http,
	types::{Log, H160, H256},
	Web3,
};
use crate::Result;
use crate::ethereum_api::get_latest_block_number;
use async_trait::async_trait;
use tokio::time::{delay_for, Duration};

#[async_trait]
pub trait LogsHandler {
	async fn handle(&self, topics_list: Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> Result<()>;
}

pub struct DefaultLogsHandler;

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
    async fn handle(&self, _topics_list: Vec<(H160, Vec<H256>)>, logs: Vec<Log>) -> Result<()> {
		println!("{:?}", logs);
		Ok(())
    }
}

#[derive(Clone)]
pub struct TopicsList<H: LogsHandler> {
	topics_list: Vec<(H160, Vec<H256>)>,
	logs_handler: H,
}

impl<H: LogsHandler> TopicsList<H> {
	pub fn new(topics_list: Vec<(H160, Vec<H256>)>, logs_handler: H) -> Self {
		TopicsList {
			topics_list,
			logs_handler
		}
	}

	pub fn get_topics_list(&self) -> Vec<(H160, Vec<H256>)> {
		self.topics_list.clone()
	}

	pub async fn handle(&self, logs: Vec<Log>) -> Result<()> {
		self.logs_handler.handle(self.topics_list.clone(), logs).await
	}
}

pub struct EthereumLikeChain<C: TrackContext, H: LogsHandler> {
	name: String,
	topics_list: TopicsList<H>,
	context: C
}

impl<C: TrackContext, H: LogsHandler> EthereumLikeChain<C, H> {
	pub fn new(name: &str, topics_list: TopicsList<H>, context: C) -> Self {
		EthereumLikeChain {
			name: name.to_string(),
			topics_list,
			context,
		}
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn get_topics_list(&self) -> Vec<(H160, Vec<H256>)> {
		self.topics_list.get_topics_list()
	}

	pub async fn next_range(&mut self, web3: &Web3<Http>) -> Result<(u64, u64)> {
		self.context.next_range(web3).await
	}

	pub async fn handle(&self, logs: Vec<Log>) -> Result<()> {
		self.topics_list.handle(logs).await
	}
}

#[async_trait]
pub trait TrackContext {
	async fn next_range(&mut self, web3: &Web3<Http>) -> Result<(u64, u64)>;
}

// Ethereum
pub struct Ethereum {
	from: u64,
}

impl Ethereum {
	pub fn new(from: u64) -> Self {
		Ethereum { from }
	}
}

#[async_trait]
impl TrackContext for Ethereum {
	async fn next_range(&mut self, web3: &Web3<Http>) -> Result<(u64, u64)> {
		let to = get_latest_block_number(web3).await?;
		if to - self.from > 10 {
			let result = (self.from, to);
			self.from = to;
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			self.next_range(web3).await
		}
	}
}

// Huobi ECO Chain
pub struct Heco {
	pub from: u64,
	pub step: u64,
}

#[async_trait]
impl TrackContext for Heco {
	async fn next_range(&mut self, web3: &Web3<Http>) -> Result<(u64, u64)> {
		let latest = get_latest_block_number(web3).await?;
		let to = if self.from + self.step >= latest {
			latest
		} else {
			self.from + self.step
		};
		if to - self.from > 5 {
			let result = (self.from, to);
			self.from = to;
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			self.next_range(web3).await
		}
	}
}

// Binance Smart Chain
pub struct Bsc {
	pub from: u64,
	pub step: u64,
}

#[async_trait]
impl TrackContext for Bsc {
	async fn next_range(&mut self, web3: &Web3<Http>) -> Result<(u64, u64)> {
		let latest = get_latest_block_number(web3).await?;
		let to = if self.from + self.step >= latest {
			latest
		} else {
			self.from + self.step
		};
		if to - self.from > 10 {
			let result = (self.from, to);
			self.from = to;
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			self.next_range(web3).await
		}
	}
}

#[test]
fn test_handler() {

}
