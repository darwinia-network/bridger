use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;

use crate::{EvmChain, EvmClient, Result};

/// Huobi ECO Chain
#[derive(Debug)]
pub struct Heco;

#[async_trait]
impl EvmChain for Heco {
    const NAME: &'static str = "Heco";

    async fn next_range(from: u64, client: &EvmClient) -> Result<(u64, u64)> {
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
            sleep(Duration::from_secs(30)).await;
            Heco::next_range(from, client).await
        }
    }
}
