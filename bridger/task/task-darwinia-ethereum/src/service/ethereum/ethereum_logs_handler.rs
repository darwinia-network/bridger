use std::time::Duration;

use lifeline::Sender;
use microkv::MicroKV;
use postage::broadcast;
use tokio::time::sleep;
use web3::types::{Log, H160, H256};

use component_darwinia_subxt::darwinia::client::Darwinia;
use evm_log_tracker::{EvmClient, LogsHandler, Result};

use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};

use crate::task::DarwiniaEthereumTask;
use bridge_traits::bridge::task::BridgeSand;

pub(crate) struct EthereumLogsHandler {
    topics_list: Vec<(H160, Vec<H256>)>,
    sender_to_relay: broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    microkv: MicroKV,
    darwinia_client: Darwinia,
}

impl EthereumLogsHandler {
    pub fn new(
        topics_list: Vec<(H160, Vec<H256>)>,
        sender_to_relay: broadcast::Sender<ToRelayMessage>,
        sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
        microkv: MicroKV,
        darwinia_client: Darwinia,
    ) -> Self {
        EthereumLogsHandler {
            topics_list,
            sender_to_relay,
            sender_to_redeem,
            microkv,
            darwinia_client,
        }
    }
}

#[async_trait]
impl LogsHandler for EthereumLogsHandler {
    async fn handle(
        &mut self,
        _client: &EvmClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        // TODO: check the address
        let ring = self.topics_list[0].1[0];
        let kton = self.topics_list[1].1[0];
        let bank = self.topics_list[2].1[0];
        let relay = self.topics_list[3].1[0];
        let register = self.topics_list[4].1[0];
        let lock = self.topics_list[4].1[0];

        // Build all transactions from logs
        let txs = build_txs(logs, ring, kton, relay, register, lock);

        if !txs.is_empty() {
            // Send block number to `Relay Service`
            for tx in &txs {
                trace!(target: DarwiniaEthereumTask::NAME, "{:?}", &tx.tx_hash);
                self.sender_to_relay
                    .send(ToRelayMessage::EthereumBlockNumber(tx.block))
                    .await?;
            }

            // Send tx to `Redeem Service`
            for tx in &txs {
                if self
                    .darwinia_client
                    .verified(tx.block_hash, tx.index)
                    .await?
                    || self
                        .darwinia_client
                        .verified_issuing(tx.block_hash, tx.index)
                        .await?
                {
                    trace!(
                        target: DarwiniaEthereumTask::NAME,
                        "This ethereum tx {:?} has already been redeemed.",
                        tx.enclosed_hash()
                    );
                } else {
                    // delay to wait for possible previous extrinsics
                    sleep(Duration::from_secs(12)).await;
                    self.sender_to_redeem
                        .send(ToRedeemMessage::EthereumTransaction(tx.clone()))
                        .await?;
                }
            }
        }

        Ok(())
    }
}

fn build_txs(
    logs: Vec<Log>,
    ring: H256,
    kton: H256,
    relay: H256,
    register: H256,
    lock: H256,
) -> Vec<EthereumTransaction> {
    let mut txs = vec![];
    txs.append(
        &mut logs
            .iter()
            .map(|l| {
                let block = l.block_number.unwrap_or_default().low_u64();
                let index = l.transaction_index.unwrap_or_default().low_u64();
                if l.topics.contains(&ring) || l.topics.contains(&kton) {
                    EthereumTransaction {
                        tx_hash: EthereumTransactionHash::Token(
                            l.transaction_hash.unwrap_or_default(),
                        ),
                        block_hash: l.block_hash.unwrap_or_default(),
                        block,
                        index,
                    }
                } else if l.topics.contains(&relay) {
                    EthereumTransaction {
                        tx_hash: EthereumTransactionHash::SetAuthorities(
                            l.transaction_hash.unwrap_or_default(),
                        ),
                        block_hash: l.block_hash.unwrap_or_default(),
                        block,
                        index,
                    }
                } else if l.topics.contains(&register) {
                    EthereumTransaction {
                        tx_hash: EthereumTransactionHash::RegisterErc20Token(
                            l.transaction_hash.unwrap_or_default(),
                        ),
                        block_hash: l.block_hash.unwrap_or_default(),
                        block,
                        index,
                    }
                } else if l.topics.contains(&lock) {
                    EthereumTransaction {
                        tx_hash: EthereumTransactionHash::RedeemErc20Token(
                            l.transaction_hash.unwrap_or_default(),
                        ),
                        block_hash: l.block_hash.unwrap_or_default(),
                        block,
                        index,
                    }
                } else {
                    EthereumTransaction {
                        tx_hash: EthereumTransactionHash::Deposit(
                            l.transaction_hash.unwrap_or_default(),
                        ),
                        block_hash: l.block_hash.unwrap_or_default(),
                        block,
                        index,
                    }
                }
            })
            .collect::<Vec<EthereumTransaction>>(),
    );
    txs
}
