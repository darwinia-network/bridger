use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use serde::{Deserialize, Serialize};

use bin_e2e::config::{BeaconChainInfoConfig, EVMChainConfig, GeneralConfig, IndexConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub general: GeneralConfig,
    pub pangoro_evm: EVMChainConfig,
    pub pangoro_substrate: PangoroSubstrateConfig,
    pub goerli: BeaconChainInfoConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PangoroSubstrateConfig {
    pub endpoint: String,
    pub private_key: String,
}

impl From<PangoroSubstrateConfig> for client_pangoro::config::ClientConfig {
    fn from(config: PangoroSubstrateConfig) -> Self {
        client_pangoro::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.private_key,
            relayer_real_account: None,
        }
    }
}

impl PangoroSubstrateConfig {
    pub async fn to_substrate_client(&self) -> color_eyre::Result<PangoroClient> {
        let config = self.clone().into();
        Ok(PangoroClientComponent::component(config).await?)
    }
}
