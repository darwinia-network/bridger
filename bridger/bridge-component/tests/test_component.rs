use bridge_component::ethereum_rpc::{EthereumRpcComponent, EthereumRpcConfig};
use bridge_component::http_client::{HttpClientComponent, HttpClientConfig};
use bridge_standard::component::BridgeComponent;
use std::sync::atomic::AtomicUsize;

#[test]
fn test_http_client_component() {
    let config = HttpClientConfig { timeout: 30 };
    let component = HttpClientComponent::new(config).unwrap();
    let _client = component.component().unwrap();
    let _config = component.config();
}

#[test]
fn test_ethereum_rpc_component() {
    let config_http_client = HttpClientConfig { timeout: 30 };
    let component_http_client = HttpClientComponent::new(config_http_client).unwrap();
    let config_ethereum_rpc = EthereumRpcConfig {
        rpc: vec!["https://mainnet.infura.io/v3/_apikey_".to_string()],
        atom: 0,
    };
    let component_ethereum =
        EthereumRpcComponent::new(config_ethereum_rpc, component_http_client).unwrap();
    let _rpc = component_ethereum.component().unwrap();
}
