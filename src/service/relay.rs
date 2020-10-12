//! Relay Service
use crate::{api::Shadow, config::Config, result::Result as BridgerResult, service::Service, Pool};
use async_trait::async_trait;
use std::{cell::RefCell, sync::Arc, time::Duration};

/// Attributes
const SERVICE_NAME: &str = "relay";

/// Relay Service
pub struct RelayService<'r> {
    step: u64,
    /// Shadow API
    pub shadow: &'r Shadow,
}

impl<'r> RelayService<'r> {
    /// New relay service
    pub async fn new(config: &'r Config, shadow: &'r Shadow) -> RelayService<'r> {
        RelayService {
            step: config.step.relay,
            shadow,
        }
    }
}

#[async_trait(?Send)]
impl<'r> Service for RelayService<'r> {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, _pool: Arc<RefCell<Pool>>) -> BridgerResult<()> {
        loop {
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
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
