use component_ethereum::ethereum::{EthereumComponent, EthereumConfig};
use component_ethereum::web3::Web3Config;

use crate::config::ShadowConfig;
use crate::error::ShadowComponentReuslt;
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
        let gql = gql_client::Client::new(shadow_config.thegraph);
        let ethereum = EthereumComponent::component(ethereum_config, web3_config)?;
        Ok(Shadow::new(shadow_config.endpoint, gql, bridge))
    }
}
