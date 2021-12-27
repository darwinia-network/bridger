use web3::transports::Http;
use web3::Web3;

use crate::ethereum::client::{EthereumClient, EthereumConfig};

pub mod client;
pub mod types;

// #[derive(Clone, Debug, Default)]
// pub struct EthereumComponent {
//     config: EthereumConfig,
//     web3_component: Web3Component,
// }

// #[async_trait::async_trait]
// impl BridgeComponent<EthereumConfig, EthereumClient> for EthereumComponent {
//     fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
//     where
//         Self: Sized,
//     {
//         let config: EthereumConfig = Config::restore_with_namespace_unwrap(T::NAME, &namespace)?;
//         let web3_component = Web3Component::restore_with_namespace::<T>(namespace)?;
//         Ok(Self::new(config, web3_component))
//     }
//
//     async fn component(&self) -> anyhow::Result<EthereumClient> {
//         let web3 = self.web3_component.component().await?;
//         Ok(EthereumClient::new(self.config.clone(), web3))
//     }
//
//     fn config(&self) -> &EthereumConfig {
//         &self.config
//     }
// }

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
