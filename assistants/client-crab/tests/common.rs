use client_crab::client::CrabClient;
use client_crab::component::CrabClientComponent;
use client_crab::config::ClientConfig;
use client_crab::error::ClientResult;

pub async fn client() -> ClientResult<CrabClient> {
    let config = ClientConfig {
        endpoint: "ws://g2.dev.darwinia.network:2244".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
    };
    CrabClientComponent::component(config).await
}
