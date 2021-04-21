use crate::Result;
use async_trait::async_trait;
use tokio::time::{delay_for, Duration};
use crate::client::EthereumLikeChainClient;

#[async_trait]
pub trait Chain {
	const NAME: &'static str;

	async fn next_range(from: u64, client: &EthereumLikeChainClient) -> Result<(u64, u64)>;
}

// Ethereum
pub struct Ethereum {
}

#[async_trait]
impl Chain for Ethereum {
	const NAME: &'static str = "Ethereum";

	async fn next_range(from: u64, client: &EthereumLikeChainClient) -> Result<(u64, u64)> {
		let to = client.get_latest_block_number().await?;
		if to - from > 10 {
			let result = (from, to);
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			Ethereum::next_range(from, client).await
		}
	}
}

// Huobi ECO Chain
pub struct Heco {
}

#[async_trait]
impl Chain for Heco {
	const NAME: &'static str = "Heco";

	async fn next_range(from: u64, client: &EthereumLikeChainClient) -> Result<(u64, u64)> {
		let latest = client.get_latest_block_number().await?;
		let to = if from + 5000 >= latest {
			latest
		} else {
			from + 5000
		};
		if to - from > 5 {
			let result = (from, to);
			Ok(result)
		} else {
			delay_for(Duration::from_secs(30)).await;
			Heco::next_range(from, client).await
		}
	}
}


