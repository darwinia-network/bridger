use relay_substrate_client::Client;

use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;

async fn client() -> anyhow::Result<Client<PangolinChain>> {
    Ok(
        relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
            host: "pangolin-rpc.darwinia.network".to_string(),
            port: 443,
            secure: true,
        })
        .await,
    )
}

pub async fn api() -> anyhow::Result<PangolinApi> {
    let client = client().await?;
    Ok(PangolinApi::new(client))
}
