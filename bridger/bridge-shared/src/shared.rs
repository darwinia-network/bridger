use std::fmt::{Debug, Formatter};

use lifeline::{Bus, Sender};

use bridge_config::config::component::BeeConfig;
use bridge_config::Config;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;

use crate::bus::SharedBus;
use crate::messages::{DarwiniaMessage, SharedMessage};
use crate::service::darwinia::{DarwiniaSharedService, SharedTask};

#[derive(Debug)]
pub struct BridgeShared {
    services: Vec<Box<dyn BridgeService>>,
    channel: SharedChannel,
}

impl BridgeShared {
    pub fn new(config: SharedConfig) -> anyhow::Result<Self> {
        config.store()?;
        let bus = SharedBus::default();

        let services = vec![Self::spawn_service::<DarwiniaSharedService>(&bus)?];

        let sender = bus.tx::<SharedMessage>()?;
        Ok(Self {
            services,
            channel: SharedChannel::new(sender),
        })
    }
}

impl BridgeShared {
    fn spawn_service<
        S: lifeline::Service<Bus = SharedBus, Lifeline = anyhow::Result<S>> + BridgeService + 'static,
    >(
        bus: &SharedBus,
    ) -> anyhow::Result<Box<dyn BridgeService>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}

impl BridgeShared {
    pub fn channel(&self) -> SharedChannel {
        self.channel.clone()
    }
}

// -- config --

#[derive(Clone, Debug)]
pub struct SharedConfig {
    pub service_darwinia: DarwiniaServiceConfig,
}

#[derive(Clone, Debug)]
pub struct DarwiniaServiceConfig {
    pub bee: BeeConfig,
}

impl DarwiniaServiceConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        Config::store(cell_name.as_ref(), self.bee.clone())?;
        Ok(())
    }
}

impl SharedConfig {
    pub fn store(&self) -> anyhow::Result<()> {
        self.service_darwinia.store(SharedTask::NAME)?;
        Ok(())
    }
}

// -- channel --

#[derive(Clone)]
pub struct SharedChannel {
    sender: postage::broadcast::Sender<SharedMessage>,
}

lifeline::impl_storage_clone!(SharedChannel);

impl Debug for SharedChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("SharedChannel { sender: <...> }")?;
        Ok(())
    }
}

impl SharedChannel {
    pub fn new(sender: postage::broadcast::Sender<SharedMessage>) -> Self {
        Self { sender }
    }
}

impl SharedChannel {
    pub async fn send(&mut self, message: SharedMessage) -> anyhow::Result<()> {
        self.sender.send(message).await?;
        Ok(())
    }
    pub async fn send_darwinia(&mut self, message: DarwiniaMessage) -> anyhow::Result<()> {
        self.send(SharedMessage::Darwinia(message)).await
    }
}
