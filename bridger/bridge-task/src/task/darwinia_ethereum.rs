use bridge_component::Component;
use bridge_config::config::component::{BeeConfig, EthereumRpcConfig, ShadowConfig, Web3Config};
use bridge_standard::bridge::task::BridgeTask;
use chain_darwinia::chain::DarwiniaChain;
use chain_ethereum::chain::EthereumChain;

use crate::bus::DarwiniaEthereumBus;
use bridge_config::Config;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    bus: <Self as BridgeTask>::Bus,
}

impl BridgeTask for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}

impl DarwiniaEthereumTask {
    pub fn with(config: DarwiniaEthereumConfig) -> anyhow::Result<Self> {
        Self::cache_config(config)?;
        Ok(Self {
            bus: Default::default(),
        })
    }

    fn cache_config(config: DarwiniaEthereumConfig) -> anyhow::Result<()> {
        let name = Self::NAME;
        config.store(name)?;
        Ok(())
    }
}

impl DarwiniaEthereumTask {
    pub fn start(self) {
        drop(self)
    }

    pub fn bus(&self) -> &<Self as BridgeTask>::Bus {
        &self.bus
    }

    pub fn spawn_service<S: lifeline::Service<Bus = <Self as BridgeTask>::Bus>>(
        &self,
    ) -> S::Lifeline {
        S::spawn(&self.bus)
    }
}

#[derive(Clone, Debug)]
pub struct DarwiniaEthereumConfig {
    pub bee: BeeConfig,
    pub web3: Web3Config,
    pub ethereum_rpc: EthereumRpcConfig,
    pub shadow: ShadowConfig,
}

impl DarwiniaEthereumConfig {
    fn store<S: AsRef<str>>(&self, task_name: S) -> anyhow::Result<()> {
        let name = task_name.as_ref();
        Config::store(name, self.bee.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum_rpc.clone())?;
        Config::store(name, self.shadow.clone())?;
        Ok(())
    }
}
