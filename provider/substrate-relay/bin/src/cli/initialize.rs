use crate::client::cli_client::CliClient;
use crate::error::Result;
use crate::types::cond::relay::InitBridgeCond;

pub async fn exec(
    server: String,
    token: Option<String>,
    source: String,
    target: String,
) -> Result<()> {
    let client = CliClient::new(server.clone(), token.clone(), false);
    let init_bridge = InitBridgeCond::builder()
        .source(source)
        .target(target)
        .build();
    client.init_bridge(&init_bridge).await?;
    Ok(())
}
