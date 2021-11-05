use component_pangolin_s2s::PangolinChain;
use relay_substrate_client::Client;

pub async fn client() -> anyhow::Result<Client<PangolinChain>> {
    Ok(
        relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
            host: "pangolin-rpc.darwinia.network".to_string(),
            port: 443,
            secure: true,
        })
        .await,
    )
}
