use client_pangolin::component::SubxtComponent;
use client_pangolin::config::{ClientConfig, PangolinSubxtConfig};
use subxt::Client;

pub async fn client() -> color_eyre::Result<Client<PangolinSubxtConfig>> {
    let config = ClientConfig {
        endpoint: "https://pangolin-rpc.darwinia-network".to_string(),
        relayer_private_key: "0x1234".to_string(),
        relayer_real_account: None,
        ecdsa_authority_private_key: None,
    };
    SubxtComponent::component(config).await
}
