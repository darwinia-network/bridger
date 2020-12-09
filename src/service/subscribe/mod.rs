//! Darwinia Subscribe
use crate::{
    api::Darwinia,
    error::Result,
    service::sign::MsgSign,
};
use primitives::{
    frame::bridge::relay_authorities::NewAuthorities,
    runtime::DarwiniaRuntime
};
use std::sync::Arc;
use substrate_subxt::EventSubscription;
use crate::error::BizError;
use actix::Recipient;
use substrate_subxt::sp_core::Decode;

// mod backing;
// mod relay;

/// Dawrinia Subscribe
pub struct SubscribeService {
    sub: EventSubscription<DarwiniaRuntime>,
    stop: bool,
    sign_service: Option<Recipient<MsgSign>>,
}

impl SubscribeService {
    /// New redeem service
    pub async fn new(darwinia: Arc<Darwinia>, sign_service: Option<Recipient<MsgSign>>) -> Result<SubscribeService> {
        Ok(SubscribeService {
            sub: darwinia.build_event_subscription().await?,
            stop: false,
            sign_service
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
                    self.handle(&event.module, &event.variant, event.data).await;
                }
            }
        }
        Ok(())
    }

    async fn handle(&mut self, module: &str, variant: &str, event_data: Vec<u8>) {
        debug!(">> Event - {}::{}", module, variant);

        if let ("EthereumRelayAuthorities", "NewAuthorities") = (module, variant) {
            if let Some(sign_service) = &self.sign_service {
                if let Ok(decoded) = NewAuthorities::<DarwiniaRuntime>::decode(&mut &event_data[..]) {
                    let msg = MsgSign { message: decoded.message };
                    if let Err(e) = sign_service.send(msg).await {
                        error!("Send msg to sign service fail: {:?}", e);
                    }
                }
            }
        }
    }
}
