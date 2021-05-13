use crate::cli::types::OptBridgeInfo;
use crate::client::cli_client::CliClient;
use crate::error::Result;
use crate::types::cond::relay::SourceAndTargetCond;

pub async fn exec(bridge_info: OptBridgeInfo) -> Result<()> {
	let server = bridge_info.server;
	let token = bridge_info.token;
	let source = bridge_info.source;
	let target = bridge_info.target;

	let client = CliClient::new(server.clone(), token.clone(), false);
	let init_bridge = SourceAndTargetCond::builder().source(source).target(target).build();
	client.init_bridge(&init_bridge).await?;
	Ok(())
}
