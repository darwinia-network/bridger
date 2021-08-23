use web3::types::{Log, H160, H256};

use crate::{EvmClient, Result};

#[async_trait]
pub trait EvmChain {
    const NAME: &'static str;

    async fn next_range(from: u64, client: &EvmClient) -> Result<(u64, u64)>;
}

#[async_trait]
pub trait LogsHandler {
    async fn handle(
        &mut self,
        from: u64,
        to: u64,
        client: &EvmClient,
        topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()>;
}
