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

    // async fn check_pending(parcel: &EthereumRelayHeaderParcel, pending_parcels: Vec<PendingRelayHeaderParcel>) {
    //     for p in pending_parcels {
    //         let parcel_p = p.2;
    //         // I disagree the parcel from pending
    //         if parcel.header.hash != parcel_p.header.hash || parcel.mmr_root != parcel_p.mmr_root {
    //             warn!("The parcel from chain is different from local calculation, please check!");
    //         }
    //     }
    // }

    async fn run(&mut self, _: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        if self.darwinia.role == Role::Normal {
            trace!("Current account is not Sudo account or technical committee, ending...");
            return Ok(());
        }

        loop {
            let last_confirmed = self.darwinia.last_confirmed().await?;
            info!("Last confirmed ethereum block number is {}", last_confirmed);

            trace!("Checking pending headers...");
            let pending_header_parcels = self.darwinia.pending_headers().await?;
            for pending_parcel in pending_header_parcels {
                let parcel = self.shadow.parcel(pending_parcel.2.header.number as usize).await?;

                if pending_parcel.2.mmr_root == parcel.mmr_root && pending_parcel.2.header.hash == parcel.header.hash {
                    info!("Approved header {}", pending_parcel.1);
                    self.darwinia.approve_pending_header(pending_parcel.1).await
                } else {
                    info!("Rejected header {}", pending_parcel.1);
                    self.darwinia.reject_pending_header(pending_parcel.1).await
                }?;
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
