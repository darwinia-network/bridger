//! Guard Service
use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    memcache::MemCache,
    result::{Error::Bridger, Result as BridgerResult},
    service::Service,
};
use async_trait::async_trait;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Attributes
const SERVICE_NAME: &str = "GUARD";

macro_rules! quick_ok {
    ($expr:expr, $err:expr) => {{
        match $expr.await {
            Ok(v) => v,
            Err(e) => {
                error!("{}, due to `{}`", $err, e);

                continue;
            }
        }
    }};
}

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
            step: config.step.guard,
        }
    }
}

#[async_trait(?Send)]
impl Service for GuardService {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, _: Arc<Mutex<MemCache>>) -> BridgerResult<()> {
        self.role_checking().await?;

        loop {
            let last_confirmed = quick_ok!(
                self.darwinia.last_confirmed(),
                "Failed to get last confirmed"
            );
            info!("Last confirmed ethereum block number is {}", last_confirmed);

            trace!("Checking pending headers...");
            let pending_headers = quick_ok!(
                self.darwinia.pending_headers(),
                "Failed to get pending headers"
            );
            for pending in pending_headers {
                if !self.darwinia.account.has_voted(pending.2) {
                    let pending_parcel = pending.1;
                    let pending_block_number: u64 = pending_parcel.header.number;
                    let parcel = quick_ok!(
                        self.shadow.parcel(pending_block_number as usize),
                        "Failed to get pendind parcel"
                    );
                    let parcel_fulfilled = !(parcel.header.hash.is_none()
                        || parcel.header.hash.unwrap() == [0u8; 32]
                        || parcel.mmr_root == [0u8; 32]);

                    if parcel_fulfilled {
                        // delay to wait for possible previous extrinsics
                        tokio::time::delay_for(Duration::from_secs(12)).await;
                        if parcel.is_same_as(&pending_parcel) {
                            quick_ok!(
                                self.darwinia
                                    .vote_pending_relay_header_parcel(pending_block_number, true),
                                "Failed to vote for pending parcel"
                            );
                            info!("Voted to approve {}", pending_block_number);
                        } else {
                            quick_ok!(
                                self.darwinia
                                    .vote_pending_relay_header_parcel(pending_block_number, false),
                                "Failed to vote for pending parcel"
                            );
                            info!("Voted to reject {}", pending_block_number);
                        };
                    }
                }
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}

impl GuardService {
    /// check permission
    pub async fn role_checking(&self) -> BridgerResult<()> {
        if !self.darwinia.account.is_tech_comm_member().await? {
            let msg = "Guard service is not running because the account is not a member of the technical committee!".to_string();
            Err(Bridger(msg))
        } else {
            Ok(())
        }
    }
}
