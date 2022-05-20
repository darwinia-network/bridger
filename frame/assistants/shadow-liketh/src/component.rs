use component_ethereum::ethereum::{EthereumComponent, EthereumConfig};
use component_ethereum::web3::Web3Config;

use crate::config::ShadowConfig;
use crate::error::{ShadowComponentError, ShadowComponentReuslt};
use crate::shadow::Shadow;
use crate::types::BridgeName;

/// Shadow component
pub struct ShadowComponent;

impl ShadowComponent {
    /// Get shadow instance
    pub fn component(
        shadow_config: ShadowConfig,
        ethereum_config: EthereumConfig,
        web3_config: Web3Config,
        bridge: BridgeName,
    ) -> ShadowComponentReuslt<Shadow> {
        let gql = gql_client::Client::new(shadow_config.thegraph.clone());
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(shadow_config.timeout))
            .build()
            .map_err(|e| ShadowComponentError::External(format!("[HTTP] {:?}", e)))?;
        let ethereum = EthereumComponent::component(ethereum_config, web3_config)
            .map_err(|e| ShadowComponentError::External(format!("[ETHEREUM] {:?}", e)))?;
        Ok(Shadow::new(
            shadow_config,
            gql,
            ethereum,
            http_client,
            bridge,
        ))
    }
}
