//! Relay Service
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
const SERVICE_NAME: &str = "RELAY";

/// Relay Service
pub struct RelayService {
    step: u64,
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
}

impl RelayService {
    /// New relay service
    pub fn new(config: &Config, shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> RelayService {
        RelayService {
            darwinia,
            shadow,
            step: config.step.relay,
        }
    }
}

#[async_trait(?Send)]
impl Service for RelayService {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, pool: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        loop {
            if let Ok(pool_clone) = pool.try_lock() {
                trace!("Checking if need to relay a new ethereum block...");
                let txs = &pool_clone.ethereum;
                if let Some(max) = txs.iter().max() {
                    let max = max.block.to_owned();
                    match self.darwinia.should_relay(max + 1).await {
                        Ok(Some(_)) => {
                            trace!("Trying to relay block {}...", max + 1);
                            let parcel = self.shadow.parcel((max + 1) as usize).await;
                            if parcel.is_err() {
                                error!("{:?}", parcel);
                                continue;
                            }

                            match self.darwinia.affirm(parcel?).await {
                                Ok(hash) => info!("Summited proposal {:?}", hash),
                                Err(err) => error!("{:?}", err),
                            }
                        }
                        Ok(None) => {}
                        Err(err) => warn!("{:?}", err),
                    }
                }

                drop(pool_clone);
            }

            // sleep
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
