use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::config::ClientConfig;
use client_pangolin::error::ClientResult;

pub async fn client() -> ClientResult<PangolinClient> {
    let config = ClientConfig {
        endpoint: "wss://pangolin-rpc.darwinia.network".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
        #[cfg(feature = "ethlike-v1")]
        ecdsa_authority_private_key: None,
    };
    PangolinClientComponent::component(config).await
}
