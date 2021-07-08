use evm_log_tracker::{EvmClient, LogsHandler, Result};
use web3::types::{Log, H160, H256};

#[derive(Debug)]
pub(crate) struct EthereumLogsHandler {

}

#[async_trait]
impl LogsHandler for EthereumLogsHandler {
    async fn handle(
        &self,
        _client: &EvmClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        for log in logs {
            info!("{:?}", log);
        }

        // tx.send(DarwiniaEthereumMessage::ToDarwinia(
        //     ToDarwiniaLinkedMessage::SendExtrinsic,
        // ))
        // .await?;
        Ok(())
    }
}