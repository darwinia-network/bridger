use getset::{Getters, Setters};
use relay_chain::types::transfer::HexLaneId;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct SourceAndTargetCond {
	source: String,
	target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct StartRelayCond {
	source: String,
	target: String,
	lance: HexLaneId,
	no_prometheus: bool,
	prometheus_host: String,
	prometheus_port: u16,
}
