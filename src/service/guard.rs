//! Guard Service
use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    result::Result as BridgerResult,
    service::Service,
    Pool,
};
use async_trait::async_trait;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Attributes
const SERVICE_NAME: &str = "GUARD";

/// Redeem Service
pub struct GuardService {
    step: u64,
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
}

impl GuardService {
    /// New redeem service
    pub fn new(config: &Config, shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> GuardService {
        GuardService {
            darwinia,
            shadow,
            step: config.step.redeem,
        }
    }
}

#[async_trait(?Send)]
impl Service for GuardService {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, _: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        loop {
            // let pending_headers = self.darwinia.pending_headers().await?;
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
