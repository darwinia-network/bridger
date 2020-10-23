//! Guard Service
use crate::{
    api::{Darwinia, Role, Shadow},
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
        if self.darwinia.role == Role::Normal {
            return Ok(());
        }

        loop {
            trace!("Checking pending headers...");
            let pending_headers = self.darwinia.pending_headers().await?;
            for header in pending_headers {
                let ht = self.shadow.parcel(header.2.header.number as usize).await?;

                if header.2 == ht {
                    info!("Approved header {}", header.1);
                    self.darwinia.approve_pending_header(header.1).await
                } else {
                    info!("Rejected header {}", header.1);
                    self.darwinia.reject_pending_header(header.1).await
                }?;
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
