use relay_substrate_client::Client;

use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;

async fn client() -> anyhow::Result<Client<PangolinChain>> {
    Ok(
        relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
            host: "127.0.0.1".to_string(),
            port: 9955,
            secure: false,
        })
        .await,
    )
}

pub async fn api() -> anyhow::Result<PangolinApi> {
    let client = client().await?;
    Ok(PangolinApi::new(client))
}
