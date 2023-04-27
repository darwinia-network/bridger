use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use serde::{Deserialize, Serialize};

use bin_e2e::config::{
    BeaconApiConfig, EVMChainConfig, ExecutionLayerInfoConfig, GeneralConfig, IndexConfig,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub general: GeneralConfig,
    pub pangolin_evm: EVMChainConfig,
    pub pangolin_substrate: PangolinSubstrateConfig,
    pub goerli: ExecutionLayerInfoConfig,
    pub beacon: BeaconApiConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PangolinSubstrateConfig {
    pub endpoint: String,
    pub private_key: String,
}

impl From<PangolinSubstrateConfig> for client_pangolin::config::ClientConfig {
    fn from(config: PangolinSubstrateConfig) -> Self {
        client_pangolin::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.private_key,
            relayer_real_account: None,
        }
    }
}

impl PangolinSubstrateConfig {
    pub async fn to_substrate_client(&self) -> color_eyre::Result<PangolinClient> {
        let config = self.clone().into();
        Ok(PangolinClientComponent::component(config).await?)
    }
}
