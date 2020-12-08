//! Darwinia Subscribe
use crate::{
    api::Darwinia,
    error::Result,
};
use primitives::runtime::DarwiniaRuntime;
use std::sync::Arc;
use substrate_subxt::EventSubscription;
use crate::error::BizError;

mod backing;
mod relay;

/// Attributes
const ETHEREUM_RELAY: &str = "EthereumRelay";
const ETHEREUM_BACKING: &str = "EthereumBacking";

/// Dawrinia Subscribe
pub struct SubscribeService {
    /// Shadow API
    pub shadow: Arc<Shadow>,
    sub: EventSubscription<DarwiniaRuntime>,
    stop: bool,
}

impl SubscribeService {
    /// New redeem service
    pub async fn new(darwinia: Arc<Darwinia>) -> Result<SubscribeService> {
        Ok(SubscribeService {
            sub: darwinia.build_event_subscription().await?,
            stop: false
        })
    }

    /// start
    pub async fn start(&mut self) -> Result<SubscribeService> {
        info!("✨ SERVICE STARTED: SUBSCRIBE");
        loop {
            if let Err(e) = self.process_next_event().await {
                if e.to_string() == "CodeUpdated" {
                    self.stop();
                    return Err(e);
                } else {
                    error!("Fail to process next event: {:?}", e);
                }
            }
            if self.stop {
                return Err(BizError::Bridger("Force stop".to_string()).into());
            }
        }
    }

    /// stop
    pub fn stop(&mut self) {
        info!("💤 SERVICE STOPPED: SUBSCRIBE");
        self.stop = true;
    }

    /// process_next_event
    async fn process_next_event(&mut self) -> Result<()> {
        if let Some(raw) = self.sub.next().await {
            if let Ok(event) = raw {
                // Remove the system events temporarily because it`s too verbose.
                if &event.module == "System" {
                    if event.variant.as_str() == "CodeUpdated" {
                        return Err(BizError::Bridger("CodeUpdated".to_string()).into());
                    }
                } else {
                    debug!(">> Event - {}::{}", &event.module, &event.variant);
                    // Common events to debug
                    match event.module.as_str() {
                        ETHEREUM_RELAY => relay::handle(event)?,
                        ETHEREUM_BACKING => backing::handle(event)?,
                        _ => {}
                    };
                }
            }
        }
        Ok(())
    }


}
