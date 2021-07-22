use web3::types::{Log, H160, H256};

use crate::{EvmClient, LogsHandler, Result};

pub struct DefaultLogsHandler;

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
    async fn handle(
        &mut self,
        _client: &EvmClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        for log in logs {
            info!("{:?}", log);
        }
        Ok(())
    }
}
