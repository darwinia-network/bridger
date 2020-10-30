//! Relay Service
use crate::{
    api::{Darwinia, Shadow},
    config::Config,
    memcache::MemCache,
    result::Error,
    result::Result as BridgerResult,
    service::Service,
};
use async_trait::async_trait;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use substrate_subxt::sp_core::H256;
use primitives::chain::ethereum::EthereumHeader;

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

    async fn run(&mut self, cache: Arc<Mutex<MemCache>>) -> BridgerResult<()> {
        loop {
            if let Ok(cache_cloned) = cache.try_lock() {
                trace!("Heartbeat>>>");
                let txs = &cache_cloned.txpool;
                if let Some(max) = txs.iter().max() {
                    let max = max.block.to_owned();
                    if let Err(err) = self.affirm(max + 1).await {
                        trace!("{:?}", err);
                    };
                }

                drop(cache_cloned);
            } else {
                error!("try_lock failed");
            }

            // sleep
            tokio::time::delay_for(Duration::from_secs(self.step)).await;
        }
    }
}

impl RelayService {
    /// affirm target block
    pub async fn affirm(&mut self, target: u64) -> BridgerResult<H256> {
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. last confirmed check
        let last_confirmed = self.darwinia.last_confirmed().await?;
        if target <= last_confirmed {
            let reason = format!(
                "The target block {} is less than the last_confirmed {}",
                &target, &last_confirmed
            );
            return Err(Error::Bridger(reason));
        }

        // 2. pendings check
        let pending_headers = self.darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                let reason = format!("The target block {} is pending", &target);
                return Err(Error::Bridger(reason));
            }
        }

        // 3. affirmations check
        for (_game_id, game) in self.darwinia.affirmations().await?.iter() {
            for (_round_id, affirmations) in game.iter() {
                if Darwinia::contains(&affirmations, target) {
                    let reason = format!("The target block {} is in the relayer game", &target);
                    return Err(Error::Bridger(reason));
                }
            }
        }

        trace!("Prepare to affirm ethereum block: {}", target);
        let parcel = self.shadow.parcel(target as usize).await?;

        if parcel.header == EthereumHeader::default()
            || parcel.mmr_root == [0u8;32]
        {
            let reason = format!("Shadow service failed to provide parcel for block {}", &target);
            return Err(Error::Bridger(reason));
        }

        // /////////////////////////
        // do affirm
        // /////////////////////////
        match self.darwinia.affirm(parcel).await {
            Ok(hash) => {
                info!("Affirmed ethereum block {} in extrinsic {:?}", target, hash);
                Ok(hash)
            }
            Err(err) => Err(err),
        }
    }
}
