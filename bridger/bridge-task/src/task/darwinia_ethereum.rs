use lifeline::{Bus, Sender};

use bridge_component::Component;
use bridge_config::config::component::{BeeConfig, EthereumRpcConfig, ShadowConfig, Web3Config};
use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_config::Config;
use bridge_standard::bridge::task::BridgeTask;
use chain_darwinia::DarwiniaChain;
use chain_ethereum::EthereumChain;
use service_darwinia_ethereum::message::s2e::EthereumScanMessage;

use crate::bus::DarwiniaEthereumBus;
use bridge_standard::bridge::service::BridgeService;

#[derive(Debug, Clone)]
pub struct DarwiniaEthereumTask {}

impl BridgeTask for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}

impl DarwiniaEthereumTask {
    pub fn with(config: DarwiniaEthereumConfig) -> anyhow::Result<DarwiniaEthereumTaskBoot> {
        config.store(Self::NAME)?;
        Ok(DarwiniaEthereumTaskBoot {
            bus: Default::default(),
            services: vec![],
        })
    }
}

#[derive(Debug)]
pub struct DarwiniaEthereumTaskBoot {
    bus: DarwiniaEthereumBus,
    services: Vec<Box<dyn BridgeService<DarwiniaEthereumTask>>>,
}

impl DarwiniaEthereumTaskBoot {
    pub async fn start(&self) -> anyhow::Result<()> {
        let mut tx_scan = self.bus.tx::<EthereumScanMessage<DarwiniaEthereumTask>>()?;
        // drop(self.bus);
        tx_scan
            .send(EthereumScanMessage::<DarwiniaEthereumTask>::Scan)
            .await?;
        Ok(())
    }

    // fixme: remove it, only test code
    pub async fn send_scan(&self) -> anyhow::Result<()> {
        let mut tx_scan = self.bus.tx::<EthereumScanMessage<DarwiniaEthereumTask>>()?;
        tx_scan
            .send(EthereumScanMessage::<DarwiniaEthereumTask>::Scan)
            .await?;
        Ok(())
    }

    pub fn bus(&self) -> &DarwiniaEthereumBus {
        &self.bus
    }

    pub fn spawn_service<
        S: lifeline::Service<Bus = DarwiniaEthereumBus, Lifeline = anyhow::Result<S>>
            + BridgeService<DarwiniaEthereumTask>
            + 'static,
    >(
        &mut self,
    ) -> anyhow::Result<&mut Self> {
        let service = S::spawn(&self.bus)?;
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
    pub service: SubstrateEthereumConfig,
}

impl DarwiniaEthereumConfig {
    pub fn store<S: AsRef<str>>(&self, task_name: S) -> anyhow::Result<()> {
        let name = task_name.as_ref();
        Config::store(name, self.bee.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum_rpc.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.service.clone())?;
        Ok(())
    }
}
