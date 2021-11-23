use relay_substrate_client::Client;

use component_darwinia_s2s::api::DarwiniaApi;
use component_darwinia_s2s::DarwiniaChain;

async fn client() -> anyhow::Result<Client<DarwiniaChain>> {
    Ok(
        relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
            host: "rpc.darwinia.network".to_string(),
            port: 443,
            secure: true,
        })
        .await,
    )
}

pub async fn api() -> anyhow::Result<DarwiniaApi> {
    let client = client().await?;
    Ok(DarwiniaApi::new(client))
}
