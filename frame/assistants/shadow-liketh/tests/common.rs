use component_ethereum::ethereum::EthereumConfig;
use component_ethereum::web3::Web3Config;
use shadow_liketh::component::ShadowComponent;
use shadow_liketh::config::ShadowConfig;
use shadow_liketh::shadow::Shadow;
use shadow_liketh::types::BridgeName;

pub enum Network {
    Ropsten,
    Ethereum,
}

impl Network {
    pub fn endpoint(&self) -> &str {
        match self {
            Network::Ropsten => "https://shadow-ropsten.darwinia.network",
            Network::Ethereum => "https://shadow-ethereum.darwinia.network",
        }
    }

    pub fn thegraph(&self) -> &str {
        match self {
            Network::Ropsten => {
                "https://api.thegraph.com/subgraphs/name/darwinia-network/ropsten-mmr"
            }
            Network::Ethereum => {
                "https://api.thegraph.com/subgraphs/name/darwinia-network/ethereum-mmr"
            }
        }
    }
}

/// Get shadow client
pub fn shadow(network: Network) -> Shadow {
    let shadow_config = ShadowConfig {
        endpoint: network.endpoint().to_string(),
        thegraph: network.thegraph().to_string(),
        timeout: 30,
    };
    let ethereum_config = EthereumConfig::default();
    let web3_config = Web3Config::default();
    ShadowComponent::component(
        shadow_config,
        ethereum_config,
        web3_config,
        BridgeName::PangolinRopsten,
    )
    .unwrap()
}
