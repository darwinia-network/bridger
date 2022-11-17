#![allow(dead_code)]
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use client_pangoro::config::ClientConfig;
use client_pangoro::error::ClientResult;

pub async fn client() -> ClientResult<PangoroClient> {
    let config = ClientConfig {
        endpoint: "wss://pangoro-rpc.darwinia.network".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
    };
    PangoroClientComponent::component(config).await
}
