use crate::cli::types::{OptRelayV2, RelayBridgeInfo};
use crate::client::cli_client::CliClient;
use crate::error;
use crate::types::cond::relay::{StartRelayCond, StatusRelayCond, StopRelayCond};

pub async fn init_bridge(bridge: RelayBridgeInfo) -> error::Result<()> {
	let source_chain_info = bridge.source_chain_info()?;
	let target_chain_info = bridge.target_chain_info()?;
	debug!("{:?}", source_chain_info);
	debug!("{:?}", target_chain_info);
	Ok(())
}

pub async fn exec(opt_relay: OptRelayV2) -> error::Result<()> {
	Ok(())
}
