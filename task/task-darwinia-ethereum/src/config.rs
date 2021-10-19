use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::{BridgeConfig, Config};
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_ethereum::config::{EthereumConfig, Web3Config};
use component_http_client::HttpClientConfig;
use component_shadow::ShadowConfig;
use component_thegraph_liketh::config::TheGraphLikeEthConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaEthereumConfig {
    pub darwinia: DarwiniaSubxtConfig,
    pub web3: Web3Config,
    pub ethereum: EthereumConfig,
    pub shadow: ShadowConfig,
    pub task: TaskConfig,
    pub http_client: HttpClientConfig,
    pub thegraph: TheGraphLikeEthConfig,
}

impl DarwiniaEthereumConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let name = cell_name.as_ref();
        Config::store(name, self.darwinia.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.task.clone())?;
        Config::store(name, self.http_client.clone())?;
        Config::store(name, self.thegraph.clone())?;
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
            thegraph: TheGraphLikeEthConfig::template(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, bridge_traits::BridgeCrypto)]
pub struct TaskConfig {
    /// the config is enable crypto
    #[crypto(is_enable)]
    pub enable_crypto: bool,
    /// ethereum scan service polling interval in seconds
    pub interval_ethereum: u64,
    /// relay service polling interval in seconds
    pub interval_relay: u64,
    /// guard service polling interval in seconds
    pub interval_guard: u64,
    /// check service polling interval in seconds
    pub interval_check: u64,
    /// timeout for check transaction (unit: seconds)
    pub check_timeout: u64,
}

impl BridgeConfig for TaskConfig {
    fn marker() -> &'static str {
        "service-substrate-ethereum"
    }

    fn template() -> Self {
        Self {
            enable_crypto: false,
            interval_ethereum: 120,
            interval_relay: 60,
            interval_guard: 30,
            interval_check: 90,
            check_timeout: (60 * 30) + 200,
        }
    }
}
