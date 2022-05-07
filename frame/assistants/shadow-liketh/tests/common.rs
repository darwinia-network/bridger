use component_ethereum::ethereum::EthereumConfig;
use component_ethereum::web3::Web3Config;
use shadow_liketh::component::ShadowComponent;
use shadow_liketh::config::ShadowConfig;
use shadow_liketh::shadow::Shadow;
use shadow_liketh::types::BridgeName;

/// Get shadow client
pub fn shadow() -> Shadow {
    let shadow_config = ShadowConfig {
        endpoint: "https://shadow.darwinia.network".to_string(),
        thegraph: "https://api.thegraph.com/subgraphs/name/darwinia-network/ropsten-mmr"
            .to_string(),
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
