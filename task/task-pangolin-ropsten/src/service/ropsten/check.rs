use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::error::StandardError;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::darwinia::client::Darwinia;
use support_tracker::Tracker;

use crate::service::EthereumTransaction;
use crate::task::PangolinRopstenTask;

type CheckerQueue = Arc<Mutex<VecDeque<Vec<EthereumTransaction>>>>;

#[derive(Clone, Debug)]
pub struct RopstenScanChecker {
    queue: CheckerQueue,
}

impl RopstenScanChecker {
    pub fn new<'a>(tracker: Tracker) -> Self {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let checker = Self {
            queue: queue.clone(),
        };
        tokio::spawn(async move { background_task(tracker, queue).await });
        checker
    }
}

async fn background_task(tracker: Tracker, queue: CheckerQueue) {
    loop {
        // Component
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        // client
        let darwinia = component_pangolin_subxt.component().await?;
        if let Err(err) = start(&tracker, &darwinia, &queue).await {
            let secs = 10;
            error!(
                target: PangolinRopstenTask::NAME,
                "ethereum err {:#?}, wait {} seconds", err, secs
            );
            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
        }
    }
}

async fn start(tracker: &Tracker, client: &Darwinia, queue: &CheckerQueue) -> anyhow::Result<()> {
    let mut queue = queue.lock().map_err(|_e| {
        StandardError::Other("failed to get waited check redeem list".to_string()).into()
    })?;
    if let Some(txs) = queue.pop_front() {
        let blocks = HashSet::from_iter(txs.iter().map(|tx| tx.block).collect::<Vec<u64>>());
        let mut verified_blocks = vec![];

        // start check
        for block in blocks {
            let mut all_verified = true;
            let txs_for_block = txs
                .iter()
                .filter(|tx| *(tx.block) == block)
                .collect::<Vec<&EthereumTransaction>>();
            for tx in txs_for_block {
                match is_verified(client, tx) {
                    Ok(false) => all_verified = false,
                    Err(e) => {
                        all_verified = false;
                        log::error!(
                            target: PangolinRopstenTask::NAME,
                            "Failed verified redeem. [{}]: {}. {:?}",
                            tx.block,
                            tx.block_hash,
                            e
                        );
                    }
                }
            }

            if !all_verified {
                continue;
            }
            verified_blocks.push(block);
        }

        // save check finish
        for block in verified_blocks {
            tracker.finish(block as usize)?;
        }
        // update queue
        let mut new_txs = txs.clone();
        new_txs.retain(|tx| !verified_blocks.contains(&tx.block));
        if !new_txs.is_empty() {
            queue.push_front(new_txs);
        }
    }
    Ok(())
}

async fn is_verified(client: &Darwinia, tx: &EthereumTransaction) -> anyhow::Result<bool> {
    Ok(client.verified(tx.block_hash, tx.index).await?
        || client.verified_issuing(tx.block_hash, tx.index).await?)
}

impl RopstenScanChecker {
    pub async fn push(&self, txs: Vec<EthereumTransaction>) -> anyhow::Result<()> {
        let mut queue = self.queue.lock().map_err(|_e| {
            StandardError::Other("failed to get waited check redeem list".to_string()).into()
        })?;
        queue.push_back(txs);
        Ok(())
    }
}
