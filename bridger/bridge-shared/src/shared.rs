use std::fmt::{Debug, Formatter};

use lifeline::{Bus, Sender};
use serde::{Deserialize, Serialize};

use bridge_component::config::BeeConfig;
use bridge_standard::bridge::config::Config;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};

use crate::bus::SharedBus;
use crate::messages::{DarwiniaMessage, SharedMessage};
use crate::service::darwinia::DarwiniaSharedService;

#[derive(Debug)]
pub struct BridgeShared {
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    channel: SharedChannel,
}

impl BridgeShared {
    pub fn new(config: SharedConfig) -> anyhow::Result<Self> {
        config.store()?;
        let bus = SharedBus::default();

        let services = vec![Self::spawn_service::<DarwiniaSharedService>(&bus)?];

        let sender = bus.tx::<SharedMessage>()?;
        let receiver = bus.rx::<SharedMessage>()?;
        Ok(Self {
            services,
            channel: SharedChannel::new(sender, receiver),
        })
    }
}

impl BridgeShared {
    fn spawn_service<
        S: lifeline::Service<Bus = SharedBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        bus: &SharedBus,
    ) -> anyhow::Result<Box<dyn BridgeService + Send + Sync>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}

impl BridgeShared {
    pub fn channel(&self) -> SharedChannel {
        self.channel.clone()
    }
}

// -- task --

#[derive(Clone, Debug)]
pub struct SharedTask {}

impl BridgeTask for SharedTask {}

impl BridgeSand for SharedTask {
    const NAME: &'static str = "task-shared";
}

// -- config --

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SharedConfig {
    pub darwinia: DarwiniaServiceConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
        self.darwinia.store(SharedTask::NAME)?;
        Ok(())
    }
}

// -- channel --

type SharedChannelTx =
    <<SharedMessage as lifeline::Message<SharedBus>>::Channel as lifeline::Channel>::Tx;
type SharedChannelRx =
    <<SharedMessage as lifeline::Message<SharedBus>>::Channel as lifeline::Channel>::Rx;

#[derive(Clone)]
pub struct SharedChannel {
    sender: SharedChannelTx,
    receiver: SharedChannelRx,
}

lifeline::impl_storage_clone!(SharedChannel);

impl Debug for SharedChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.write_str("SharedChannel { sender: <...> }")?;
        Ok(())
    }
}

impl SharedChannel {
    pub fn new(sender: SharedChannelTx, receiver: SharedChannelRx) -> Self {
        Self { sender, receiver }
    }
}

impl SharedChannel {
    pub fn sender(&self) -> SharedChannelTx {
        self.sender.clone()
    }
    pub fn receiver(&self) -> SharedChannelRx {
        self.receiver.clone()
    }
    pub async fn send(&mut self, message: SharedMessage) -> anyhow::Result<()> {
        self.sender.send(message).await?;
        Ok(())
    }
    pub async fn send_darwinia(&mut self, message: DarwiniaMessage) -> anyhow::Result<()> {
        self.send(SharedMessage::Darwinia(message)).await
    }
}
