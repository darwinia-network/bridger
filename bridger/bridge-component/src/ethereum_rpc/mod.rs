use bridge_standard::component::BridgeComponent;
use bridge_standard::config::BridgeConfig;

use crate::http_client::HttpClientComponent;

pub use self::rpc::*;

mod block;
mod receipt;
mod rpc;

#[derive(Clone, Debug, Default)]
pub struct EthereumRpcConfig {
    /// Rpc host
    pub rpc: Vec<String>,
    /// Counter
    pub atom: usize,
}

impl BridgeConfig for EthereumRpcConfig {}

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

impl BridgeComponent<EthereumRpcConfig, EthereumRpc> for EthereumRpcComponent {
    fn component(&self) -> anyhow::Result<EthereumRpc> {
        let client = self.http_client_component.component()?;
        Ok(EthereumRpc::new(client, self.config.clone()))
    }

    fn config(&self) -> &EthereumRpcConfig {
        &self.config
    }
}
