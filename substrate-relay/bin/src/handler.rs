use chain_relay::s2s::init_bridge::InitBridge;
use chain_relay::types::transfer::{HexLaneId, RelayHeadersAndMessagesInfo};

use crate::types::{PrometheusParams, RelayBridgeInfo};

pub async fn init_bridge(bridge: RelayBridgeInfo) -> anyhow::Result<()> {
	let source_chain_info = bridge.source_chain_info()?;
	let target_chain_info = bridge.target_chain_info()?;

	let init_bridge = InitBridge::new(bridge.bridge().clone(), source_chain_info, target_chain_info);

	chain_relay::s2s::init_bridge::run(init_bridge).await
}

pub async fn on_demand_relay(
	bridge: RelayBridgeInfo,
	lanes: Vec<HexLaneId>,
	prometheus: PrometheusParams,
) -> anyhow::Result<()> {
	let source_chain_info = bridge.source_chain_info()?;
	let target_chain_info = bridge.target_chain_info()?;

	let mut relay_info = RelayHeadersAndMessagesInfo::default();
	relay_info.set_source(source_chain_info);
	relay_info.set_target(target_chain_info);
	relay_info.set_lanes(lanes);
	relay_info.set_prometheus_params(prometheus.prometheus_info());

	chain_relay::s2s::relay_headers_and_messages::run(relay_info).await
}
