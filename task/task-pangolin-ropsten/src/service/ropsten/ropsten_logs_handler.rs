use lifeline::{Receiver, Sender};
use postage::broadcast;
use web3::types::{Log, H160, H256};

use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::darwinia::client::Darwinia;
use support_tracker::Tracker;
use support_tracker_evm_log::{EvmClient, LogsHandler, Result};

use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use crate::task::PangolinRopstenTask;

pub(crate) struct RopstenLogsHandler {
    topics_list: Vec<(H160, Vec<H256>)>,
    sender_to_relay: broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    receiver_redeem: broadcast::Receiver<ToRedeemMessage>,
    darwinia_client: Darwinia,
    tracker: Tracker,
}

impl RopstenLogsHandler {
    pub fn new(
        topics_list: Vec<(H160, Vec<H256>)>,
        sender_to_relay: broadcast::Sender<ToRelayMessage>,
        sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
        receiver_redeem: broadcast::Receiver<ToRedeemMessage>,
        darwinia_client: Darwinia,
        tracker: Tracker,
    ) -> Self {
        RopstenLogsHandler {
            topics_list,
            sender_to_relay,
            sender_to_redeem,
            receiver_redeem,
            darwinia_client,
            tracker,
        }
    }
}

#[async_trait]
impl LogsHandler for RopstenLogsHandler {
    async fn handle(
        &mut self,
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
    async fn redeem(&mut self, tx: &EthereumTransaction) -> anyhow::Result<()> {
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
                target: PangolinRopstenTask::NAME,
                "This ethereum tx {:?} has already been redeemed.",
                tx.enclosed_hash()
            );
            self.tracker.finish(tx.block as usize)?;
        } else {
            trace!(
                target: PangolinRopstenTask::NAME,
                "send to redeem service: {:?}",
                &tx.tx_hash
            );
            self.sender_to_redeem
                .send(ToRedeemMessage::EthereumTransaction(tx.clone()))
                .await?;
            while let Some(ToRedeemMessage::Complete(block)) = self.receiver_redeem.recv().await {
                let except = tx.block as usize;
                if block != except {
                    continue;
                }
                self.tracker.finish(block)?;
                break;
            }
        }

        Ok(())
    }
}
