use web3::types::{Log, H160, H256};

use crate::{DefaultLogsHandler, EvmClient, EvmLogRangeData, LogsHandler};

impl<'a> EvmLogRangeData<'a> {
    pub fn new(
        from: u64,
        to: u64,
        client: &'a EvmClient,
        topics: &'a Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Self {
        Self {
            from,
            to,
            client,
            topics,
            logs,
        }
    }
}

impl<'a> EvmLogRangeData<'a> {}

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
    async fn handle(&mut self, data: EvmLogRangeData) -> anyhow::Result<()> {
        for log in logs {
            info!("{:?}", log);
        }
        Ok(())
    }
}
