use serde::{Deserialize, Serialize};

use client_pangolin::config::ClientConfig;
use component_ethereum::ethereum::EthereumConfig;
use component_ethereum::web3::Web3Config;
use component_http_client::HttpClientConfig;
use component_shadow::config::ShadowConfig;
use subquery_d2e::SubqueryConfig;
use thegraph_liketh::config::TheGraphLikeEthConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinRopstenConfig {
    pub darwinia: ClientConfig,
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
