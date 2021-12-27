use serde::{Deserialize, Serialize};
use web3::transports::Http;
use web3::Web3;

use crate::ethereum::client::EthereumClient;
use crate::web3::{Web3Component, Web3Config};

pub mod client;
pub mod types;

/// Ethereum provider
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Ethereum relayer private key
    pub relayer_private_key: Option<String>,
    /// The darwinia account key
    pub relayer_beneficiary_darwinia_account: Option<String>,
    /// Contract relay address
    pub subscribe_relay_address: String,
}

impl Default for EthereumConfig {
    fn default() -> Self {
        Self {
            relayer_private_key: Some("0x...".to_string()),
            relayer_beneficiary_darwinia_account: Some("0x...".to_string()),
            subscribe_relay_address: "0x...".to_string(),
        }
    }
}

/// Ethereum component
pub struct EthereumComponent;

impl EthereumComponent {
    /// Get ethereum client instance
    pub fn component(
        ethereum_config: EthereumConfig,
        web3_config: Web3Config,
    ) -> color_eyre::Result<EthereumClient> {
        let web3 = Web3Component::component(web3_config)?;
        Ok(EthereumClient::new(ethereum_config, web3))
    }
}
