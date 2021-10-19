use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::EthereumConfig;
use crate::ethereum::client::EthereumClient;
use crate::web3::Web3Component;

pub mod client;
pub mod types;

#[derive(Clone, Debug, Default)]
pub struct EthereumComponent {
    config: EthereumConfig,
    web3_component: Web3Component,
}

impl EthereumComponent {
    pub fn new(config: EthereumConfig, web3_component: Web3Component) -> Self {
        Self {
            config,
            web3_component,
        }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<EthereumConfig, EthereumClient> for EthereumComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
    where
        Self: Sized,
    {
        let config: EthereumConfig = Config::restore_with_namespace(T::NAME, &namespace)?;
        let web3_component = Web3Component::restore_with_namespace::<T>(namespace)?;
        Ok(Self::new(config, web3_component))
    }

    async fn component(&self) -> anyhow::Result<EthereumClient> {
        let web3 = self.web3_component.component().await?;
        Ok(EthereumClient::new(self.config.clone(), web3))
    }

    fn config(&self) -> &EthereumConfig {
        &self.config
    }
}
