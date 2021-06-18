use bridge_component::ethereum_rpc::{EthereumRpcComponent, EthereumRpcConfig};
use bridge_component::http_client::{HttpClientComponent, HttpClientConfig};
use bridge_standard::component::BridgeComponent;
use std::sync::atomic::AtomicUsize;

mod common;

#[test]
fn test_http_client_component() {
    let component = common::component_http_client_default();
    let _config = component.config();
}

#[test]
fn test_ethereum_rpc_component() {
    let component = common::component_ethereum_rpc_default("api_key");
    let _config = component.config();
}

#[test]
fn test_shadow_component() {
    let component = common::component_shadow_default("api_key");
    let _config = component.config();
}
