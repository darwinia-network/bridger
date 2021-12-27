use web3::transports::Http;
use web3::Web3;

use crate::ethereum::client::{EthereumClient, EthereumConfig};

pub mod client;
pub mod types;

/// Ethereum provider
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthereumProvider {
    /// Ethereum relayer private key
    pub relayer_private_key: Option<String>,
    /// The darwinia account key
    pub relayer_beneficiary_darwinia_account: Option<String>,
    /// Contract relay address
    pub subscribe_relay_address: String,
}

impl EthereumProvider {
    /// Get ethereum client instance
    pub fn component(&self, web3: Web3<Http>) -> color_eyre::Result<EthereumClient> {
        let config = EthereumConfig {
            relayer_private_key: self.relayer_private_key.clone(),
            relayer_beneficiary_darwinia_account: self.relayer_beneficiary_darwinia_account.clone(),
            subscribe_relay_address: self.subscribe_relay_address.clone(),
        };
        Ok(EthereumClient::new(config, web3))
    }
}
