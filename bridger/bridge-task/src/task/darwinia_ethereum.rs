use lifeline::{Bus, Sender};

use bridge_component::Component;
use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_config::Config;
use bridge_shared::channel::SharedChannel;
use bridge_shared::traits::SharedService;
use bridge_standard::bridge::sand::BridgeSand;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeTask;
use chain_darwinia::DarwiniaChain;
use chain_ethereum::EthereumChain;
use service_darwinia_ethereum::message::s2e::EthereumScanMessage;
use service_darwinia_ethereum::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use service_darwinia_ethereum::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

use crate::bus::DarwiniaEthereumBus;

#[derive(Debug, Clone)]
pub struct DarwiniaEthereumTask {}

impl BridgeSand for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
}

impl BridgeTask for DarwiniaEthereumTask {
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}

impl DarwiniaEthereumTask {
    pub fn new(
        config: DarwiniaEthereumConfig,
        channel: SharedChannel,
    ) -> anyhow::Result<DarwiniaEthereumTaskBoot> {
        config.store(Self::NAME)?;
        Ok(DarwiniaEthereumTaskBoot {
            bus: Self::bus(),
            services: vec![],
            channel,
        })
    }
}

#[derive(Debug)]
pub struct DarwiniaEthereumTaskBoot {
    bus: DarwiniaEthereumBus,
    services: Vec<Box<dyn BridgeService>>,
    channel: SharedChannel,
}

impl DarwiniaEthereumTaskBoot {
    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService<DarwiniaEthereumTask>>()?;
        self.spawn_shared_service::<LikeDarwiniaWithLikeEthereumEthereumScanService<DarwiniaEthereumTask>>()?;

        let mut tx_scan = self.bus.tx::<EthereumScanMessage<DarwiniaEthereumTask>>()?;
        // drop(self.bus);
        tx_scan
            .send(EthereumScanMessage::<DarwiniaEthereumTask>::Start)
            .await?;
        Ok(())
    }

    // fixme: remove it, only test code
    pub async fn send_scan(&self) -> anyhow::Result<()> {
        let mut tx_scan = self.bus.tx::<EthereumScanMessage<DarwiniaEthereumTask>>()?;
        tx_scan
            .send(EthereumScanMessage::<DarwiniaEthereumTask>::Start)
            .await?;
        Ok(())
    }

    pub fn bus(&self) -> &DarwiniaEthereumBus {
        &self.bus
    }

    fn spawn_service<
        S: lifeline::Service<Bus = DarwiniaEthereumBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + 'static,
    >(
        &mut self,
    ) -> anyhow::Result<&mut Self> {
        let service = S::spawn(&self.bus)?;
        self.services.push(Box::new(service));
        Ok(self)
    }

    fn spawn_shared_service<
        S: BridgeService
            + SharedService<Bus = DarwiniaEthereumBus, Lifeline = anyhow::Result<S>>
            + 'static,
    >(
        &mut self,
    ) -> anyhow::Result<&mut Self> {
        let service = S::spawn(&self.bus, self.channel.clone())?;
        self.services.push(Box::new(service));
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct DarwiniaEthereumConfig {
    pub bee: BeeConfig,
    pub web3: Web3Config,
    pub ethereum_rpc: EthereumRpcConfig,
    pub shadow: ShadowConfig,
    pub microkv: MicrokvConfig,
    pub service: SubstrateEthereumConfig,
}

impl DarwiniaEthereumConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let name = cell_name.as_ref();
        Config::store(name, self.bee.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum_rpc.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.service.clone())?;
        Config::store(name, self.microkv.clone())?;
        Ok(())
    }
}
