use bridge_config::component::EthereumRpcConfig;
use bridge_standard::bridge::component::BridgeComponent;

use crate::http_client::HttpClientComponent;

pub use self::rpc::*;

mod block;
mod receipt;
mod rpc;

#[derive(Clone, Debug, Default)]
pub struct EthereumRpcComponent {
    config: EthereumRpcConfig,
    http_client_component: HttpClientComponent,
}

impl EthereumRpcComponent {
    pub fn new(
        config: EthereumRpcConfig,
        http_client_component: HttpClientComponent,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            http_client_component,
        })
    }
}

#[async_trait]
impl BridgeComponent<EthereumRpcConfig, EthereumRpc> for EthereumRpcComponent {
    async fn component(&self) -> anyhow::Result<EthereumRpc> {
        let client = self.http_client_component.component().await?;
        Ok(EthereumRpc::new(client, self.config.clone()))
    }

    fn config(&self) -> &EthereumRpcConfig {
        &self.config
    }
}
