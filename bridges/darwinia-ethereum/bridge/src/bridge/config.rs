use client_darwinia::client::DarwiniaClient;
use client_darwinia::component::DarwiniaClientComponent;
use serde::{Deserialize, Serialize};

use bin_e2e::config::{BeaconChainInfoConfig, EVMChainConfig, GeneralConfig, IndexConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub general: GeneralConfig,
    pub darwinia_evm: EVMChainConfig,
    pub darwinia_substrate: DarwiniaSubstrateConfig,
    pub eth: BeaconChainInfoConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DarwiniaSubstrateConfig {
    pub endpoint: String,
    pub private_key: String,
}

impl From<DarwiniaSubstrateConfig> for client_darwinia::config::ClientConfig {
    fn from(config: DarwiniaSubstrateConfig) -> Self {
        client_darwinia::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.private_key,
            relayer_real_account: None,
        }
    }
}

impl DarwiniaSubstrateConfig {
    pub async fn to_substrate_client(&self) -> color_eyre::Result<DarwiniaClient> {
        let config = self.clone().into();
        Ok(DarwiniaClientComponent::component(config).await?)
    }
}
