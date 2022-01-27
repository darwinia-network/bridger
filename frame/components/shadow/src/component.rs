use component_ethereum::ethereum::{EthereumComponent, EthereumConfig};
use component_ethereum::web3::Web3Config;

use crate::config::ShadowConfig;
use crate::shadow::Shadow;

/// Shadow component
pub struct ShadowComponent;

impl ShadowComponent {
    /// Get shadow instance
    pub fn component(
        shadow_config: ShadowConfig,
        ethereum_config: EthereumConfig,
        web3_config: Web3Config,
    ) -> color_eyre::Result<Shadow> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        let ethereum = EthereumComponent::component(ethereum_config, web3_config)?;
        Ok(Shadow::new(shadow_config, http_client, ethereum))
    }
}
