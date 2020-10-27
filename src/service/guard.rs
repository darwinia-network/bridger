//! Guard Service
use crate::{
    api::{Darwinia, Shadow, darwinia::{contains_role, ROLE_TECHNICAL_COMMITTEE}},
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
use crate::result::Error::Bridger;

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
        self.role_checking()?;

        loop {
            let last_confirmed = self.darwinia.last_confirmed().await?;
            info!("Last confirmed ethereum block number is {}", last_confirmed);

            trace!("Checking pending headers...");
            let pending_headers = self.darwinia.pending_headers().await?;
            for pending in pending_headers {
                let pending_parcel = pending.1;
                let pending_block_number: u64 = pending_parcel.header.number;
                let parcel = self.shadow.parcel(pending_block_number as usize).await?;

                if parcel.is_same_as(&pending_parcel) {
                    self.darwinia.vote_pending_relay_header_parcel(pending_block_number, true).await?;
                    info!("Voted to approve {}", pending_block_number);
                } else {
                    self.darwinia.vote_pending_relay_header_parcel(pending_block_number, false).await?;
                    info!("Voted to reject {}", pending_block_number);
                };
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}

impl GuardService {
    /// check permission
    pub fn role_checking(&self) -> BridgerResult<()> {
        if !contains_role(self.darwinia.roles, ROLE_TECHNICAL_COMMITTEE.1) {
            let msg = "Guard service is not running because the relayer is not a member of the technical committee!".to_string();
            Err(Bridger(msg))
        } else {
            Ok(())
        }
    }
}
