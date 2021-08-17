use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::{BridgeConfig, Config};
use component_ethereum::config::{EthereumConfig, Web3Config};
use component_http_client::HttpClientConfig;
use component_pangolin_subxt::config::DarwiniaSubxtConfig;
use component_shadow::ShadowConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinRopstenConfig {
    pub darwinia: DarwiniaSubxtConfig,
    pub web3: Web3Config,
    pub ethereum: EthereumConfig,
    pub shadow: ShadowConfig,
    pub task: TaskConfig,
    pub http_client: HttpClientConfig,
}

impl PangolinRopstenConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let name = cell_name.as_ref();
        Config::store(name, self.darwinia.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.task.clone())?;
        Config::store(name, self.http_client.clone())?;
        Ok(())
    }
    pub fn template() -> Self {
        Self {
            darwinia: DarwiniaSubxtConfig::template(),
            web3: Web3Config::template(),
            ethereum: EthereumConfig::template(),
            shadow: ShadowConfig::template(),
            task: TaskConfig::template(),
            http_client: HttpClientConfig::template(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskConfig {
    /// ethereum scan service polling interval in seconds
    pub interval_ethereum: u64,
    /// relay service polling interval in seconds
    pub interval_relay: u64,
    /// redeem service polling interval in seconds
    pub interval_redeem: u64,
    /// guard service polling interval in seconds
    pub interval_guard: u64,
}

impl BridgeConfig for TaskConfig {
    fn marker() -> &'static str {
        "service-substrate-ethereum"
    }

    fn template() -> Self {
        Self {
            interval_ethereum: 120,
            interval_relay: 60,
            interval_redeem: 90,
            interval_guard: 30,
        }
    }
}
