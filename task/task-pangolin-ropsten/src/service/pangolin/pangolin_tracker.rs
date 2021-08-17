use std::time::Duration;

use substrate_subxt::sp_runtime::generic::Header;
use substrate_subxt::sp_runtime::traits::BlakeTwo256;
use tokio::time::sleep;

use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::darwinia::client::Darwinia;
use support_tracker::Tracker;

use crate::error::Result;
use crate::task::PangolinRopstenTask;

/// DarwiniaTracker
pub struct PangolinBlockTracker {
    darwinia: Darwinia,
    tracker: Tracker,
}

impl PangolinBlockTracker {
    /// new
    pub fn new(darwinia: Darwinia, tracker: Tracker) -> Self {
        Self { darwinia, tracker }
    }

    /// get next block
    pub async fn next_block(&self) -> Result<Header<u32, BlakeTwo256>> {
        loop {
            match self.get_next_block().await {
                Ok(result) => {
                    if let Some(header) = result {
                        return Ok(header);
                    } else {
                        sleep(Duration::from_secs(6)).await;
                    }
                }
                Err(err) => {
                    error!(
                        target: PangolinRopstenTask::NAME,
                        "An error occurred while tracking next darwinia block: {:#?}", err
                    );
                    let err_msg = format!("{:?}", err).to_lowercase();
                    if err_msg.contains("restart") {
                        return Err(crate::error::Error::RestartFromJsonrpsee.into());
                    } else {
                        sleep(Duration::from_secs(30)).await;
                    }
                }
            }
        }
    }

    async fn get_next_block(&self) -> Result<Option<Header<u32, BlakeTwo256>>> {
        let next_block = self.tracker.next().await? as u32;
        let finalized_block_hash = self.darwinia.finalized_head().await?;
        match self
            .darwinia
            .get_block_number_by_hash(finalized_block_hash)
            .await?
        {
            Some(finalized_block_number) => {
                if next_block > finalized_block_number {
                    Ok(None)
                } else {
                    let header = self.darwinia.get_block_by_number(next_block).await?;
                    Ok(Some(header))
                }
            }
            None => Ok(None),
        }
    }
}
