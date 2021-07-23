use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;

use crate::{EvmChain, EvmClient, Result};

/// Ethereum
#[derive(Debug)]
pub struct Ethereum;

#[async_trait]
impl EvmChain for Ethereum {
    const NAME: &'static str = "Ethereum";

    async fn next_range(from: u64, client: &EvmClient) -> Result<(u64, u64)> {
        let to = client.get_latest_block_number().await?;
        if to - from > 10 {
            let result = (from, to);
            Ok(result)
        } else {
            sleep(Duration::from_secs(30)).await;
            Ethereum::next_range(from, client).await
        }
    }
}
