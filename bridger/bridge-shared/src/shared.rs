use lifeline::Bus;

use crate::bus::SharedBus;
use crate::channel::SharedChannel;
use crate::config::SharedConfig;
use crate::messages::SharedMessage;
use crate::service::darwinia::DarwiniaSharedService;
use crate::traits::SharedKeepService;

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
    fn spawn_service<
        S: lifeline::Service<Bus = SharedBus, Lifeline = anyhow::Result<S>>
            + SharedKeepService
            + 'static,
    >(
        &mut self,
    ) -> anyhow::Result<&mut Self> {
        let service = S::spawn(&self.bus)?;
        self.services.push(Box::new(service));
        Ok(self)
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.config.store()?;
        self.spawn_service::<DarwiniaSharedService>()?;
        Ok(())
    }

    pub fn channel(&self) -> anyhow::Result<SharedChannel> {
        let sender = self.bus.tx::<SharedMessage>()?;
        Ok(SharedChannel::new(sender))
    }
}
