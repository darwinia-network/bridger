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
use primitives::{chain::ethereum::RedeemFor};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

/// Attributes
const SERVICE_NAME: &str = "REDEEM";

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
            if let Ok(mut pool_clone) = pool.try_lock() {
                trace!("Looking for redeemable ethereum transactions...");
                trace!(
                    "Currently we have {} txs might need to be redeemed",
                    pool_clone.ethereum.len(),
                );
                let last = self.darwinia.last_confirmed().await?;
                let mut redeemed = vec![];
                for index in 0..pool_clone.ethereum.len() {
                    if index >= pool_clone.ethereum.len() {
                        break;
                    }
                    let tx = &pool_clone.ethereum[index];

                    if !self.darwinia.should_redeem(&tx).await? {
                        info!("This ethereum tx {:?} has already been redeemed.", tx.tx_hash);

                        continue;
                    }

                    if tx.block < last {
                        info!("Prepare to redeem ethereum tx {:?}", tx.tx_hash);
                        let proof = self
                            .shadow
                            .receipt(&format!("{:?}", tx.enclosed_hash()), last)
                            .await?;
                        let redeem_for = match tx.tx_hash {
                            EthereumTransactionHash::Deposit(_) => RedeemFor::Deposit,
                            EthereumTransactionHash::Token(_) => RedeemFor::Token,
                        };
                        let hash = self.darwinia.redeem(redeem_for, proof).await?;
                        info!("Redeemed with extrinsic {:?}", hash);
                        redeemed.push(index);
                    }
                }

                // sleep
                pool_clone.ethereum = pool_clone
                    .ethereum
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| !redeemed.contains(idx))
                    .map(|(_, tx)| tx.clone())
                    .collect();

                if !redeemed.is_empty() {
                    trace!(
                        "Currently we have {} txs might need to be redeemed",
                        pool_clone.ethereum.len(),
                    );
                }
                drop(pool_clone);
            }

            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}
