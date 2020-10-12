//! Relay Service
use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    result::Result as BridgerResult,
    service::Service,
    Pool,
};
use async_trait::async_trait;
use std::{cell::RefCell, sync::Arc, time::Duration};

/// Attributes
const SERVICE_NAME: &str = "relay";

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

    async fn run(&mut self, pool: Arc<RefCell<Pool>>) -> BridgerResult<()> {
        loop {
            tokio::time::delay_for(Duration::from_secs(self.step)).await;

            let last = self
                .darwinia
                .last_confirmed()
                .await
                .unwrap_or(Some(0))
                .unwrap_or(0);
            info!("The last confirmed block is {:?}", last);

            // Try to relay
            if let Some(max) = pool.borrow_mut().ethereum.iter().max() {
                if max.block > last {
                    let parcel = self.shadow.proposal(last, max.block + 1, max.block).await;
                    if parcel.is_err() {
                        error!("{:?}", parcel);
                        continue;
                    }

                    match self.darwinia.submit_proposal(vec![parcel?]).await {
                        Ok(hash) => info!("Summited proposal {:?}", hash),
                        Err(err) => error!("{:?}", err),
                    }
                }
            }
        }
        // let eth = self.web3.eth();
        // let mut block_number: u64;
        // let mut start = self.start;
        //
        // loop {
        //     block_number = eth.block_number().await?.as_u64();
        //     if block_number == start {
        //         tokio::time::delay_for(Duration::from_secs(30)).await;
        //         continue;
        //     }
        //
        //     let mut txs = self.scan(start, block_number).await?;
        //     info!("Found {} txs from {} to {}", txs.len(), start, block_number);
        //     pool.borrow_mut().eth.append(&mut txs);
        //     start = block_number;
        // }
    }
}
