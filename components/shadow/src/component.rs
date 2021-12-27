use component_ethereum::ethereum::{EthereumComponent, EthereumConfig};
use component_ethereum::web3::Web3Config;
use component_http_client::{HttpClientComponent, HttpClientConfig};

use crate::{Shadow, ShadowConfig};

/// Shadow component
pub struct ShadowComponent;

impl ShadowComponent {
    /// Get shadow instance
    pub fn component(
        shadow_config: ShadowConfig,
        http_client_config: HttpClientConfig,
        ethereum_config: EthereumConfig,
        web3_config: Web3Config,
    ) -> color_eyre::Result<Shadow> {
        let http_client = HttpClientComponent::component(http_client_config)?;
        let ethereum = EthereumComponent::component(ethereum_config, web3_config)?;
        Ok(Shadow::new(shadow_config, http_client, ethereum))
    }
}
