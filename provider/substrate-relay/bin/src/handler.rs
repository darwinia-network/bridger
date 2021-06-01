use chain_relay::types::transfer::{HexLaneId, InitBridge, RelayHeadersAndMessagesInfo};

use crate::types::{PrometheusParams, RelayBridgeInfo};

pub async fn init_bridge(bridge: RelayBridgeInfo) -> anyhow::Result<()> {
	let source_chain_info = bridge.source_chain_info()?;
	let target_chain_info = bridge.target_chain_info()?;

	let init_bridge = InitBridge::new(bridge.bridge, source_chain_info, target_chain_info);

	chain_relay::s2s::init_bridge::run(init_bridge).await
}

pub async fn on_demand_relay(
	bridge: RelayBridgeInfo,
	lanes: Vec<HexLaneId>,
	prometheus: PrometheusParams,
) -> anyhow::Result<()> {
	let source_chain_info = bridge.source_chain_info()?;
	let target_chain_info = bridge.target_chain_info()?;

	let relay_info = RelayHeadersAndMessagesInfo::new(
		bridge.bridge,
		source_chain_info,
		target_chain_info,
		lanes,
		prometheus.prometheus_info(),
	);

	chain_relay::s2s::relay_headers_and_messages::run(relay_info).await
}
