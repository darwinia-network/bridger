use getset::{Getters, Setters};
use relay_chain::types::transfer::{HexLaneId, PrometheusParamsInfo};
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
	lanes: String, // fixme: there need support multiple lanes need use Vec<String>
	no_prometheus: bool,
	prometheus_host: String,
	prometheus_port: u16,
}

impl StartRelayCond {
	pub fn prometheus_info(&self) -> PrometheusParamsInfo {
		let mut prometheus_info = PrometheusParamsInfo::default();
		prometheus_info.set_no_prometheus(self.no_prometheus);
		prometheus_info.set_prometheus_host(self.prometheus_host.clone());
		prometheus_info.set_prometheus_port(self.prometheus_port);
		prometheus_info
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct StopRelayCond {
	source: String,
	target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct StatusRelayCond {
	source: String,
	target: String,
}
