use std::sync::atomic::{AtomicUsize, Ordering};

use reqwest::Client;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;
use component_http_client::HttpClientComponent;

use crate::client::rpc::EthereumRpcClient;
use crate::config::EthereumConfig;

pub mod rpc;

#[derive(Clone, Debug, Default)]
pub struct EthereumComponent {
    config: EthereumConfig,
    http_client_component: HttpClientComponent,
}

impl EthereumComponent {
    pub fn new(config: EthereumConfig, http_client_component: HttpClientComponent) -> Self {
        Self {
            config,
            http_client_component,
        }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<EthereumConfig, EthereumClient> for EthereumComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: EthereumConfig = Config::restore_with_namespace(T::NAME, &namespace)?;
        let http_client_component = HttpClientComponent::restore_with_namespace::<T>(namespace)?;
        Ok(Self::new(config, http_client_component))
    }

    async fn component(&self) -> anyhow::Result<EthereumClient> {
        let client = self.http_client_component.component().await?;
        Ok(EthereumClient::new(client, self.config.clone()))
    }

    fn config(&self) -> &EthereumConfig {
        &self.config
    }
}

/// Ethereum rpc set
pub struct EthereumClient {
    /// Reqwest client
    rpc_client: EthereumRpcClient,
}

impl EthereumClient {
    pub fn new(client: Client, config: EthereumConfig) -> Self {
        let rpc_client = EthereumRpcClient::new(client, config.clone());
        Self { rpc_client }
    }
}

impl EthereumClient {
    pub fn rpc(&self) -> &EthereumRpcClient {
        &self.rpc_client
    }
}
