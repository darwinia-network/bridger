use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;
use component_ethereum::ethereum::EthereumComponent;
use component_http_client::HttpClientComponent;

pub use self::config::*;
pub use self::shadow::*;

mod config;
mod shadow;

#[derive(Clone, Debug, Default)]
pub struct ShadowComponent {
    config: ShadowConfig,
    http_client_component: HttpClientComponent,
    ethereum_component: EthereumComponent,
}

impl ShadowComponent {
    pub fn new(
        config: ShadowConfig,
        http_client_component: HttpClientComponent,
        ethereum_component: EthereumComponent,
    ) -> Self {
        Self {
            config,
            http_client_component,
            ethereum_component,
        }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<ShadowConfig, Shadow> for ShadowComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: ShadowConfig = Config::restore_with_namespace(T::NAME, &namespace)?;
        let component_http_client =
            HttpClientComponent::restore_with_namespace::<T>(namespace.clone())?;
        let component_ethereum = EthereumComponent::restore_with_namespace::<T>(namespace)?;
        Ok(Self::new(config, component_http_client, component_ethereum))
    }

    async fn component(&self) -> anyhow::Result<Shadow> {
        let http_client = self.http_client_component.component().await?;
        let ethereum_client = self.ethereum_component.component().await?;
        Ok(Shadow::new(
            self.config.clone(),
            http_client,
            ethereum_client,
        ))
    }

    fn config(&self) -> &ShadowConfig {
        &self.config
    }
}
