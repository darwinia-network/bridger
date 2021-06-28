use lifeline::{Bus, Service};

use bridge_standard::bridge::sand::BridgeSand;

use crate::bus::SharedBus;
use crate::channel::SharedChannel;
use crate::config::{DarwiniaServiceConfig, SharedConfig};
use crate::material::MaterialDarwinia;
use crate::messages::SharedMessage;
use crate::service::darwinia::DarwiniaSharedService;
use crate::traits::{SharedKeepService, SharedMaterial};

#[derive(Debug)]
pub struct BridgeShared {
    config: SharedConfig,
    bus: SharedBus,
    services: Vec<Box<dyn SharedKeepService>>,
}

impl BridgeShared {
    pub fn new(config: SharedConfig) -> Self {
        BridgeShared {
            config,
            bus: SharedBus::default(),
            services: vec![],
        }
    }
}

impl BridgeShared {
    pub fn start(&mut self) -> anyhow::Result<()> {
        self.config.store_darwinia::<MaterialDarwinia>()?;

        let service_darwinia = DarwiniaSharedService::spawn(&self.bus)?;
        self.services.push(Box::new(service_darwinia));
        Ok(())
    }

    pub fn channel(&self) -> anyhow::Result<SharedChannel> {
        let sender = self.bus.tx::<SharedMessage>()?;
        Ok(SharedChannel::new(sender))
    }
}
