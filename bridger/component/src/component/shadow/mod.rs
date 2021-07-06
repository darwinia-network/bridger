use bridge_traits::bridge::component::BridgeComponent;

use crate::component::ethereum_rpc::EthereumRpcComponent;
use crate::component::http_client::HttpClientComponent;
use crate::config::ShadowConfig;

pub use self::shadow_raw::*;

mod shadow_raw;

#[derive(Clone, Debug, Default)]
pub struct ShadowComponent {
    config: ShadowConfig,
    http_client_component: HttpClientComponent,
    ethereum_rpc_component: EthereumRpcComponent,
}

impl ShadowComponent {
    pub fn new(
        config: ShadowConfig,
        http_client_component: HttpClientComponent,
        ethereum_rpc_component: EthereumRpcComponent,
    ) -> Self {
        Self {
            config,
            http_client_component,
            ethereum_rpc_component,
        }
    }
}

#[async_trait]
impl BridgeComponent<ShadowConfig, Shadow> for ShadowComponent {
    async fn component(&self) -> anyhow::Result<Shadow> {
        let http_client = self.http_client_component.component().await?;
        let ethereum_rpc = self.ethereum_rpc_component.component().await?;
        Ok(Shadow::new(self.config.clone(), http_client, ethereum_rpc))
    }

    fn config(&self) -> &ShadowConfig {
        &self.config
    }
}
