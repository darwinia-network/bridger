use bridge_e2e_traits::client::EcdsaClient;
use support_tracker::Tracker;

use crate::{
    ecdsa::{
        collected_enough_authorities_change_signatures::CollectedEnoughAuthoritiesChangeSignaturesRunner,
        collected_enough_new_message_root_signatures::CollectedEnoughNewMessageRootSignaturesRunner,
        collecting_authorities_change_signatures::CollectingAuthoritiesChangeSignaturesRunner,
        collecting_new_message_root_signatures::CollectingNewMessageRootSignaturesRunner,
    },
    error::{RelayError, RelayResult},
};

use super::types::EcdsaSource;

#[derive(Clone, Copy, Debug)]
pub enum EcdsaScanType {
    CollectingMessage,
    CollectedMessage,
    CollectingAuthority,
    CollectedAuthority,
}

pub struct EcdsaScanner;

impl EcdsaScanner {
    pub async fn start<T: EcdsaClient>(
        &self,
        tracker: Tracker,
        scan_type: EcdsaScanType,
        source: EcdsaSource<T>,
        minimal_interval: u64,
    ) {
        while let Err(err) = self
            .run(tracker.clone(), scan_type, source.clone(), minimal_interval)
            .await
        {
            tracing::error!(
                target: "pangoro-goerli",
                "[pangoro] [ecdsa] An error occurred while processing the extrinsics: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run<T: EcdsaClient>(
        &self,
        tracker: Tracker,
        scan_type: EcdsaScanType,
        mut source: EcdsaSource<T>,
        minimal_interval: u64,
    ) -> RelayResult<()> {
        loop {
            let from = tracker
                .current()
                .await
                .map_err(|e| RelayError::Custom(format!("{}", e)))?;
            tracing::info!(
                target: "pangoro-goerli",
                "[pangoro] [ecdsa] Track pangoro scan block: {} ",
                from,
            );
            source.block = Some(from as u32);

            let finished_block = match scan_type {
                EcdsaScanType::CollectingMessage => {
                    let runner = CollectingNewMessageRootSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
                EcdsaScanType::CollectedMessage => {
                    let mut runner = CollectedEnoughNewMessageRootSignaturesRunner::new(
                        source.clone(),
                        minimal_interval,
                    );
                    runner.start().await?
                }
                EcdsaScanType::CollectingAuthority => {
                    let runner = CollectingAuthoritiesChangeSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
                EcdsaScanType::CollectedAuthority => {
                    let runner =
                        CollectedEnoughAuthoritiesChangeSignaturesRunner::new(source.clone());
                    runner.start().await?
                }
            };
            if finished_block.is_some() {
                tracker
                    .finish(finished_block.unwrap() as usize)
                    .map_err(|e| RelayError::Custom(format!("{}", e)))?;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
}
