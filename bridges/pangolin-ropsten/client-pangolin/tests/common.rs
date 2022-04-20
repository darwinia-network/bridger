use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::config::ClientConfig;

pub async fn client() -> color_eyre::Result<PangolinClient> {
    let config = ClientConfig {
        endpoint: "wss://pangolin-rpc.darwinia.network".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
        ecdsa_authority_private_key: None,
    };
    PangolinClientComponent::component(config).await
}
