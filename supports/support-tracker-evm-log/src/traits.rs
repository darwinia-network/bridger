use std::fmt::Debug;

use web3::types::{Log, H160, H256};

use crate::{EvmClient, Result};

#[async_trait]
pub trait EvmChain {
    const NAME: &'static str;

    async fn next_range(from: u64, client: &EvmClient) -> Result<(u64, u64)>;
}

#[async_trait]
pub trait LogsHandler: Clone + Debug {
    async fn handle(&mut self, data: EvmLogRangeData) -> Result<()>;
}

/// Default evm logs handler
/// Will bee print all logs to console
#[derive(Clone, Debug)]
pub struct DefaultLogsHandler;

/// Queried range data of evm logs
#[derive(Clone, Debug)]
pub struct EvmLogRangeData<'a> {
    pub(crate) from: u64,
    pub(crate) to: u64,
    pub(crate) client: &'a EvmClient,
    pub(crate) topics: &'a Vec<(H160, Vec<H256>)>,
    pub(crate) logs: Vec<Log>,
}
