use evm_log_tracker::{EvmClient, LogsHandler, Result};
use web3::types::{Log, H160, H256};

#[derive(Debug)]
pub(crate) struct EthereumLogsHandler {
    topics_list: Vec<(H160, Vec<H256>)>
}

impl EthereumLogsHandler {
    pub fn new(topics_list: Vec<(H160, Vec<H256>)>) -> Self {
        EthereumLogsHandler {
            topics_list
        }
    }
}

#[async_trait]
impl LogsHandler for EthereumLogsHandler {
    async fn handle(
        &self,
        _client: &EvmClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        let ring = self.topics_list[0].1[0];
        let kton = self.topics_list[1].1[0];
        let bank = self.topics_list[2].1[0];
        let relay = self.topics_list[3].1[0];
        let register = self.topics_list[4].1[0];
        let lock = self.topics_list[4].1[1];
        for log in logs {
            let block = log.block_number.unwrap_or_default().low_u64();
            let index = log.transaction_index.unwrap_or_default().low_u64();
            if log.topics.contains(&ring) || log.topics.contains(&kton) {
                println!("Token cross-chain topic found");
            } else if log.topics.contains(&relay) {
                println!("SetAuthorities cross-chain topic found");
            } else if log.topics.contains(&register) {
                println!("RegisterErc20Token cross-chain topic found");
            } else if log.topics.contains(&lock) {
                println!("RedeemErc20Token cross-chain topic found");
            } else {
                println!("Deposit cross-chain topic found"); 
            }
        }

        // tx.send(DarwiniaEthereumMessage::ToDarwinia(
        //     ToDarwiniaLinkedMessage::SendExtrinsic,
        // ))
        // .await?;
        Ok(())
    }
}