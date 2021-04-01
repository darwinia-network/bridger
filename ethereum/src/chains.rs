use web3::{Web3, transports::Http};
use crate::Result;
use crate::ethereum_api::get_latest_block_number;
use async_trait::async_trait;
use tokio::time::{delay_for, Duration};

#[async_trait]
pub trait Chain {
	fn name(&self) -> String;
	async fn next_range(&mut self) -> Result<(u64, u64)>;
}

// Ethereum
pub struct Ethereum {
	pub web3: Web3<Http>,
	pub from: u64,
}

#[async_trait]
impl Chain for Ethereum {
	fn name(&self) -> String {
		"Ethereum".to_owned()
	}

	async fn next_range(&mut self) -> Result<(u64, u64)> {
		let to = get_latest_block_number(&self.web3).await?;
		if to - self.from > 10 {
			let result = (self.from, to);
			self.from = to;
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			self.next_range().await
		}
	}
}

// Huobi ECO Chain
pub struct Heco {
	pub web3: Web3<Http>,
	pub from: u64,
	pub step: u64,
}

#[async_trait]
impl Chain for Heco {
	fn name(&self) -> String {
		"Huobi ECO Chain".to_owned()
	}

	async fn next_range(&mut self) -> Result<(u64, u64)> {
		let latest = get_latest_block_number(&self.web3).await?;
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
			self.next_range().await
		}
	}
}

// Binance Smart Chain
pub struct Bsc {
	pub web3: Web3<Http>,
	pub from: u64,
	pub step: u64,
}

#[async_trait]
impl Chain for Bsc {
	fn name(&self) -> String {
		"Binance Smart Chain".to_owned()
	}

	async fn next_range(&mut self) -> Result<(u64, u64)> {
		let latest = get_latest_block_number(&self.web3).await?;
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
			self.next_range().await
		}
	}
}
