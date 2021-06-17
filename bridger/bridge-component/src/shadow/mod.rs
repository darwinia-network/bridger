use bridge_standard::config::BridgeConfig;

use crate::ethereum_rpc::EthereumRpcComponent;
use crate::http_client::HttpClientComponent;

pub use self::shadow::*;

mod shadow;

#[derive(Clone, Debug, Default)]
pub struct ShadowConfig {
    pub endpoint: String,
}

impl BridgeConfig for ShadowConfig {}

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
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            http_client_component,
            ethereum_rpc_component,
        })
    }
}
