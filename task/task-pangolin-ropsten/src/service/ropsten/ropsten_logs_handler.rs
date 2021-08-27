use lifeline::Sender;
use postage::broadcast;
use std::collections::{HashMap, HashSet};
use web3::types::{Log, H160, H256};

use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::darwinia::client::Darwinia;
use support_tracker::Tracker;
use support_tracker_evm_log::{EvmClient, LogsHandler, Result};

use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use crate::task::PangolinRopstenTask;

const MAX_WAITED_REDEEM_COUNT: usize = 1024;

pub(crate) struct RopstenLogsHandler {
    topics_list: Vec<(H160, Vec<H256>)>,
    sender_to_relay: broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    darwinia_client: Darwinia,
    // this waited_redeem is used to check the finished redeem block
    // maybe some transactions are in the same block. The block is confirmed when all the
    // transactions in this block are confirmed. And the queue must be sorted by block number in
    // order to avoid the missing of the older block.
    waited_redeem: HashMap<u64, HashSet<EthereumTransaction>>,
    tracker: Tracker,
}

impl RopstenLogsHandler {
    pub fn new(
        topics_list: Vec<(H160, Vec<H256>)>,
        sender_to_relay: broadcast::Sender<ToRelayMessage>,
        sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
        darwinia_client: Darwinia,
        tracker: Tracker,
    ) -> Self {
        RopstenLogsHandler {
            topics_list,
            sender_to_relay,
            sender_to_redeem,
            darwinia_client,
            waited_redeem: HashMap::new(),
            tracker,
        }
    }
}

#[async_trait]
impl LogsHandler for RopstenLogsHandler {
    async fn handle(
        &mut self,
        from: u64,
        to: u64,
        _client: &EvmClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        // TODO: check the address
        let bank_topic = self.topics_list[0].1[0];
        let issuing_topic = self.topics_list[1].1[0];
        let relay_topic = self.topics_list[2].1[0];
        let erc20_register_topic = self.topics_list[3].1[0];
        let erc20_lock_topic = self.topics_list[3].1[1];

        // Build all transactions from logs
        let txs = build_txs(
            logs,
            bank_topic,
            issuing_topic,
            relay_topic,
            erc20_register_topic,
            erc20_lock_topic,
        );

        // if [from, to] blocks has no transactions we need, and the waited_redeem queue is empty.
        // next time we only need to scan from `to` block. Other wise we need to scan from `from-1`
        // block to ensure all the transactions in interval [from, to] to be checked.
        let check_position = if txs.is_empty() { to } else { from - 1 };
        self.check_redeem(check_position).await?;
        if self.waited_redeem.len() >= MAX_WAITED_REDEEM_COUNT {
            info!(
                target: PangolinRopstenTask::NAME,
                "achieve redeem waited limit, pause scan, from: {:?}",
                &from
            );
            self.tracker.flush_current(from as usize)?;
            return Ok(());
        }

        if !txs.is_empty() {
            // Send block number to `Relay Service`
            for tx in &txs {
                trace!(
                    target: PangolinRopstenTask::NAME,
                    "{:?} in ethereum block {}",
                    &tx.tx_hash,
                    &tx.block
                );
                self.sender_to_relay
                    .send(ToRelayMessage::EthereumBlockNumber(tx.block + 1))
                    .await?;
            }

            // Send tx to `Redeem Service`
            for tx in &txs {
                self.redeem(tx).await?;
            }
        }

        self.tracker.flush_current(to as usize)?;
        Ok(())
    }
}

fn build_txs(
    logs: Vec<Log>,
    bank_topic: H256,
    issuing_topic: H256,
    relay_topic: H256,
    erc20_register_topic: H256,
    erc20_lock_topic: H256,
) -> Vec<EthereumTransaction> {
    let mut txs = vec![];
    for l in &logs {
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
                tx_hash: EthereumTransactionHash::Deposit(l.transaction_hash.unwrap_or_default()),
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
            error!(
                target: PangolinRopstenTask::NAME,
                "Can not find any useful topics in the log: {:?}", l.topics
            );
            continue;
        };

        txs.push(tx);
    }
    txs
}

impl RopstenLogsHandler {
    async fn is_verified(&self, tx: &EthereumTransaction) -> anyhow::Result<bool> {
        Ok(self
            .darwinia_client
            .verified(tx.block_hash, tx.index)
            .await?
            || self
                .darwinia_client
                .verified_issuing(tx.block_hash, tx.index)
                .await?)
    }

    fn is_redeem_submitted(&self, tx: &EthereumTransaction) -> bool {
        let txset = self.waited_redeem.get(&tx.block);
        if let Some(txs) = txset {
            return txs.contains(&tx);
        }
        false
    }

    async fn check_redeem(&mut self, check_position: u64) -> anyhow::Result<()> {
        if self.waited_redeem.is_empty() {
            trace!(
                target: PangolinRopstenTask::NAME,
                "no redeem waited, change last redeem to {:?}",
                check_position
            );
            self.tracker.finish(check_position as usize)?;
            Ok(())
        } else {
            let mut block_numbers = self.waited_redeem.keys().copied().collect::<Vec<_>>();
            block_numbers.sort_unstable();
            for redeemed in block_numbers {
                let checked = self.waited_redeem.remove(&redeemed);
                if let Some(txs) = checked {
                    let mut reverted = txs.clone();
                    for tx in txs.iter() {
                        match self.is_verified(&tx).await {
                            Err(err) => {
                                error!(
                                    target: PangolinRopstenTask::NAME,
                                    "check-redeem err: {:#?}", err
                                );
                                self.waited_redeem.insert(redeemed, reverted);
                                return Err(err);
                            }
                            Ok(verified) => {
                                if verified {
                                    info!(
                                        target: PangolinRopstenTask::NAME,
                                        "check-redeem verified tx {:?} at {:?}",
                                        tx.tx_hash,
                                        tx.block
                                    );
                                    reverted.remove(&tx);
                                } else {
                                    trace!(
                                        target: PangolinRopstenTask::NAME,
                                        "check-redeem not verified tx {:?} at {:?}",
                                        tx.tx_hash,
                                        tx.block
                                    );
                                    self.waited_redeem.insert(redeemed, reverted);
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                info!(
                    target: PangolinRopstenTask::NAME,
                    "new redeem confirmed, change last redeem to {:?}", redeemed
                );
                self.tracker.finish(redeemed as usize)?;
            }

            Ok(())
        }
    }

    async fn redeem(&mut self, tx: &EthereumTransaction) -> anyhow::Result<()> {
        if self.is_redeem_submitted(&tx) {
            return Ok(());
        }
        if self.is_verified(&tx).await? {
            trace!(
                target: PangolinRopstenTask::NAME,
                "This ethereum tx {:?} has already been redeemed.",
                tx.enclosed_hash()
            );
        } else {
            trace!(
                target: PangolinRopstenTask::NAME,
                "send to redeem service: {:?}",
                &tx.tx_hash
            );
            self.sender_to_redeem
                .send(ToRedeemMessage::EthereumTransaction(tx.clone()))
                .await?;
            trace!("finished to send to redeem: {:?}", &tx.tx_hash);
        }
        self.waited_redeem
            .entry(tx.block)
            .or_insert_with(HashSet::new)
            .insert(tx.clone());

        Ok(())
    }
}
