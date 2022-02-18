use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use client_pangoro::config::ClientConfig;
use web3::transports::Http;
use web3::Web3;

pub async fn client() -> color_eyre::Result<PangoroClient> {
    let config = ClientConfig {
        endpoint: "wss://pangoro-rpc.darwinia.network:443".to_string(),
        relayer_private_key: "//Alice".to_string(),
        relayer_real_account: None,
    };
    PangoroClientComponent::component(config).await
}

pub fn web3_client() -> color_eyre::Result<Web3<Http>> {
    let transport = web3::transports::Http::new("https://data-seed-prebsc-2-s3.binance.org:8545/")?;
    Ok(web3::Web3::new(transport))
}
