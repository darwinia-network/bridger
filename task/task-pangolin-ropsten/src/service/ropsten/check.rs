use bridge_traits::bridge::component::BridgeComponent;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::darwinia::client::Darwinia;
use support_tracker::Tracker;

use crate::task::PangolinRopstenTask;

pub struct RopstenScanChecker {
    tracker: Tracker,
}

impl RopstenScanChecker {
    pub fn new(tracker: Tracker) -> Self {
        Self { tracker }
    }
}

impl RopstenScanChecker {
    pub async fn start(&self) {
        loop {
            if let Err(e) = self.run().await {
                log::error!("Failed to check redeem: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1));
            }
        }
    }

    async fn run(&self) -> anyhow::Result<()> {
        // Component
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;

        // Darwinia client
        let darwinia = component_pangolin_subxt.component().await?;

        let records = self.tracker.parallel_records()?;
        for block in records {
            let verified = match self.is_verified(&darwinia, block).await {
                Ok(v) => v,
                Err(e) => {
                    // todo: save error times of block
                    log::error!("Failed to get block verified state: [{}] {:?}", block, e);
                    false
                }
            };
            if !verified {
                continue;
            }
            if let Err(e) = self.tracker.finish(block) {
                log::error!("Failed to set finish to kv: [{}] {:?}", block, e);
            }
        }
        Ok(())
    }

    async fn is_verified(&self, darwinia: &Darwinia, block: usize) -> anyhow::Result<bool> {
        Ok(self
            .darwinia_client
            .verified(tx.block_hash, tx.index)
            .await?
            || self
                .darwinia_client
                .verified_issuing(tx.block_hash, tx.index)
                .await?)
    }
}
