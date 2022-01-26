use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::config::ClientConfig;

pub async fn client() -> color_eyre::Result<PangolinClient> {
    let config = ClientConfig {
        endpoint: "https://pangolin-rpc.darwinia.network".to_string(),
        relayer_private_key: "0xd385c38d56fbb9704371a81a5a5d4a4575fcabfa37ce57c19ac3dce96a9d7383"
            .to_string(),
        relayer_real_account: None,
        ecdsa_authority_private_key: None,
    };
    PangolinClientComponent::component(config).await
}
