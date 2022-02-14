use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use client_pangoro::config::ClientConfig;

pub async fn client() -> color_eyre::Result<PangoroClient> {
    let config = ClientConfig {
        endpoint: "ws://127.0.0.1:9944".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
        ecdsa_authority_private_key: None,
    };
    PangoroClientComponent::component(config).await
}
