//! Redeem Service

use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    pool::EthereumTransactionHash,
    result::Result as BridgerResult,
    service::Service,
    Pool,
};
use async_trait::async_trait;
use primitives::{chain::eth::RedeemFor, hex};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Attributes
const SERVICE_NAME: &str = "redeem";

/// Redeem Service
pub struct RedeemService {
    step: u64,
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
}

impl RedeemService {
    /// New redeem service
    pub fn new(config: &Config, shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> RedeemService {
        RedeemService {
            darwinia,
            shadow,
            step: config.step.redeem,
        }
    }
}

#[async_trait(?Send)]
impl Service for RedeemService {
    fn name<'e>(&self) -> &'e str {
        SERVICE_NAME
    }

    async fn run(&mut self, pool: Arc<Mutex<Pool>>) -> BridgerResult<()> {
        loop {
            trace!("Looking for redeemable ethereum transactions...");
            let mut pool_clone = pool.lock().unwrap();
            trace!(
                "Currently we have {} txs need to be redeemed",
                pool_clone.ethereum.len()
            );
            let last = self.darwinia.last_confirmed().await?;
            for index in 0..pool_clone.ethereum.len() {
                let tx = &pool_clone.ethereum[index];
                // if self.darwinia.should_redeem(&tx).await? {
                let proof = self
                    .shadow
                    .receipt(&format!("0x{}", hex!(&tx.hash()).as_str()), last)
                    .await?;
                let redeem_for = match tx.hash {
                    EthereumTransactionHash::Deposit(_) => RedeemFor::Deposit,
                    EthereumTransactionHash::Token(_) => RedeemFor::Token,
                };
                let hash = self.darwinia.redeem(redeem_for, proof).await?;
                info!("Redeemed tx {}", hash);

                pool_clone.ethereum.remove(index);
                // }
            }

            // sleep
            drop(pool_clone);
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
