//! Relay Service
use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    result::Result as BridgerResult,
    result::Error,
    service::Service,
    Pool,
};
use async_trait::async_trait;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use substrate_subxt::sp_core::H256;

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
                    if let Err(err) = self.affirm(max + 1).await {
                        error!("{:?}", err);
                        continue;
                    };
                }

                drop(pool_clone);
            }

            // sleep
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}

impl RelayService {
    /// affirm target block
    pub async fn affirm(&mut self, target: u64) -> BridgerResult<H256> {

        match self.darwinia.should_relay(target).await {
            Ok(true) => {
                trace!("Trying to relay block {}...", target);
                let parcel = self.shadow.parcel(target as usize).await?;

                match self.darwinia.affirm(parcel).await {
                    Ok(hash) => {
                        info!("Summited proposal {:?}", hash);
                        Ok(hash)
                    },
                    Err(err) => Err(err),
                }
            }
            Ok(false) => {
                Err(Error::Etc(format!("No need to affirm block {}.", target)))
            },
            Err(err) => Err(err),
        }
    }
}
