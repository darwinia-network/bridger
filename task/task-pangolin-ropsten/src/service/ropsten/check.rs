use support_tracker::Tracker;

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
            }
            tokio::time::sleep(std::time::Duration::from_secs(1));
        }
    }

    async fn run(&self) -> anyhow::Result<()> {
        let records = self.tracker.parallel_records()?;
        for block in records {
            // todo: check redeem and remove it from kv db
        }
        Ok(())
    }
}
