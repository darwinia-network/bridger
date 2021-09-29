use web3::types::{Log, H160, H256};

use crate::{DefaultLogsHandler, EvmClient, EvmLogRangeData, LogsHandler};
use support_ethereum::transaction::{EthereumTransaction, EthereumTransactionHash};

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

impl<'a> EvmLogRangeData<'a> {
    pub fn from(&self) -> u64 {
        self.from
    }
    pub fn to(&self) -> u64 {
        self.to
    }
    pub fn topics(&self) -> &Vec<(H160, Vec<H256>)> {
        self.topics
    }
    pub fn logs(&self) -> &Vec<Log> {
        &self.logs
    }
}

impl<'a> EvmLogRangeData<'a> {
    pub fn transactions(&self) -> Vec<EthereumTransaction> {
        let bank_topic = self.topics[0].1[0];
        let issuing_topic = self.topics[1].1[0];
        let relay_topic = self.topics[2].1[0];
        let erc20_register_topic = self.topics[3].1[0];
        let erc20_lock_topic = self.topics[3].1[1];

        let mut txs = vec![];
        for l in &self.logs {
            let block = l.block_number.unwrap_or_default().low_u64();
            let index = l.transaction_index.unwrap_or_default().low_u64();
            let tx = if l.topics.contains(&issuing_topic) {
                EthereumTransaction {
                    tx_hash: EthereumTransactionHash::Token(l.transaction_hash.unwrap_or_default()),
                    block_hash: l.block_hash.unwrap_or_default(),
                    block,
                    index,
                }
            } else if l.topics.contains(&relay_topic) {
                EthereumTransaction {
                    tx_hash: EthereumTransactionHash::SetAuthorities(
                        l.transaction_hash.unwrap_or_default(),
                    ),
                    block_hash: l.block_hash.unwrap_or_default(),
                    block,
                    index,
                }
            } else if l.topics.contains(&bank_topic) {
                EthereumTransaction {
                    tx_hash: EthereumTransactionHash::Deposit(
                        l.transaction_hash.unwrap_or_default(),
                    ),
                    block_hash: l.block_hash.unwrap_or_default(),
                    block,
                    index,
                }
            } else if l.topics.contains(&erc20_register_topic) {
                EthereumTransaction {
                    tx_hash: EthereumTransactionHash::RegisterErc20Token(
                        l.transaction_hash.unwrap_or_default(),
                    ),
                    block_hash: l.block_hash.unwrap_or_default(),
                    block,
                    index,
                }
            } else if l.topics.contains(&erc20_lock_topic) {
                EthereumTransaction {
                    tx_hash: EthereumTransactionHash::RedeemErc20Token(
                        l.transaction_hash.unwrap_or_default(),
                    ),
                    block_hash: l.block_hash.unwrap_or_default(),
                    block,
                    index,
                }
            } else {
                log::error!("Can not find any useful topics in the log: {:?}", l.topics);
                continue;
            };

            txs.push(tx);
        }
        txs
    }
}

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
    async fn handle(&mut self, data: EvmLogRangeData) -> anyhow::Result<()> {
        for log in data.logs() {
            log::info!("{:?}", log);
        }
        Ok(())
    }
}
