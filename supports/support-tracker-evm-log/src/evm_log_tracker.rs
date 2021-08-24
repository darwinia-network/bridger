use std::marker::PhantomData;
use std::time::Duration;

use tokio::time::sleep;
use web3::types::{Log, H160, H256};

use support_tracker::Tracker;

use crate::{EvmChain, EvmClient, LogsHandler, Result};

#[derive(Debug)]
pub struct EvmLogTracker<C: EvmChain, H: LogsHandler> {
    client: EvmClient,
    topics_list: Vec<(H160, Vec<H256>)>,
    logs_handler: H,
    step_in_secs: u64,
    tracker: Tracker,
    running: bool,
    phantom: PhantomData<C>,
}

impl<C: EvmChain, H: LogsHandler> EvmLogTracker<C, H> {
    pub fn new(
        client: EvmClient,
        topics_list: Vec<(H160, Vec<H256>)>,
        logs_handler: H,
        tracker: Tracker,
        step_in_secs: u64,
    ) -> Self {
        EvmLogTracker {
            client,
            topics_list,
            logs_handler,
            step_in_secs,
            tracker,
            running: false,
            phantom: PhantomData,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        loop {
            match self.next().await {
                Err(err) => {
                    return Err(err);
                }
                Ok((from, to, logs)) => {
                    self.handle(from, to, logs).await?;
                }
            }

            if !self.running {
                break;
            }

            sleep(Duration::from_secs(self.step_in_secs)).await;
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub async fn next(&mut self) -> Result<(u64, u64, Vec<Log>)> {
        let mut result = vec![];
        let from = self.tracker.next().await?;
        let (from, to) = C::next_range(from as u64, &self.client).await?;
        info!(
            "Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
            C::NAME,
            from,
            to
        );
        for topics in &self.topics_list {
            let logs = self.client.get_logs(&topics.0, &topics.1, from, to).await?;
            result.extend_from_slice(&logs);
        }
        Ok((from, to, result))
    }

    async fn handle(&mut self, from: u64, to: u64, logs: Vec<Log>) -> Result<()> {
        self.logs_handler
            .handle(from, to, &self.client, &self.topics_list, logs)
            .await?;
        Ok(())
    }
}