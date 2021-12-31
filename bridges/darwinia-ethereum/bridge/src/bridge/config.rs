use serde::{Deserialize, Serialize};

use client_darwinia::config::DarwiniaSubxtConfig;
use component_ethereum::ethereum::EthereumConfig;
use component_ethereum::web3::Web3Config;
use component_http_client::HttpClientConfig;
use component_shadow::ShadowConfig;
use component_subquery::SubqueryConfig;
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
    pub subquery: SubqueryConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskConfig {
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
