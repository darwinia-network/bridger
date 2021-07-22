use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;
use component_http_client::HttpClientComponent;

use crate::config::EthereumConfig;

pub use self::rpc::*;

mod block;
mod receipt;
mod rpc;

#[derive(Clone, Debug, Default)]
pub struct EthereumRpcComponent {
    config: EthereumConfig,
    http_client_component: HttpClientComponent,
}

impl EthereumRpcComponent {
    pub fn new(config: EthereumConfig, http_client_component: HttpClientComponent) -> Self {
        Self {
            config,
            http_client_component,
        }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<EthereumConfig, EthereumRpc> for EthereumRpcComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: EthereumConfig = Config::restore_with_namespace(T::NAME, &namespace)?;
        let http_client_component = HttpClientComponent::restore_with_namespace::<T>(namespace)?;
        Ok(Self::new(config, http_client_component))
    }

    async fn component(&self) -> anyhow::Result<EthereumRpc> {
        let client = self.http_client_component.component().await?;
        Ok(EthereumRpc::new(client, self.config.clone()))
    }

    fn config(&self) -> &EthereumConfig {
        &self.config
    }
}
