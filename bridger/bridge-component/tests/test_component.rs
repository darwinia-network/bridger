use bridge_component::ethereum_rpc::{EthereumRpcComponent, EthereumRpcConfig};
use bridge_component::http_client::{HttpClientComponent, HttpClientConfig};
use bridge_standard::bridge::component::BridgeComponent;
use std::sync::atomic::AtomicUsize;

mod common;

#[async_std::test]
async fn test_http_client_component() {
    let component = common::component_http_client_default();
    let _obj = component.component().await.unwrap();
}

#[async_std::test]
async fn test_ethereum_rpc_component() {
    let component = common::component_ethereum_rpc_default("api_key");
    let _obj = component.component().await.unwrap();
}

#[async_std::test]
async fn test_shadow_component() {
    let component = common::component_shadow_default("api_key");
    let _obj = component.component().await.unwrap();
}

#[async_std::test]
async fn test_web3_component() {
    let component = common::component_web3_default("api_key");
    let _obj = component.component().await.unwrap();
}

#[async_std::test]
async fn test_bee_component() {
    let component = common::component_bee::<common::TestChainTypes>();
    let _obj = component.component().await.unwrap();
}
