use crate::types::transfer::ChainInfo;
use bp_messages::LaneId;
use getset::{Getters, MutGetters, Setters};

#[derive(Debug, Clone, Default, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct RelayHeadersAndMessagesInfo {
	source: ChainInfo,
	target: ChainInfo,

	lanes: Vec<HexLaneId>,
	prometheus_params: PrometheusParamsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HexLaneId(pub LaneId);

impl From<HexLaneId> for LaneId {
	fn from(lane_id: HexLaneId) -> LaneId {
		lane_id.0
	}
}

impl std::str::FromStr for HexLaneId {
	type Err = hex::FromHexError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lane_id = LaneId::default();
		hex::decode_to_slice(s, &mut lane_id)?;
		Ok(HexLaneId(lane_id))
	}
}

#[derive(Debug, Clone, Default, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct PrometheusParamsInfo {
	/// Do not expose a Prometheus metric endpoint.
	no_prometheus: bool,
	/// Expose Prometheus endpoint at given interface.
	prometheus_host: String,
	/// Expose Prometheus endpoint at given port.
	prometheus_port: u16,
}

impl From<PrometheusParamsInfo> for relay_utils::metrics::MetricsParams {
	fn from(cli_params: PrometheusParamsInfo) -> relay_utils::metrics::MetricsParams {
		if !cli_params.no_prometheus {
			Some(relay_utils::metrics::MetricsAddress {
				host: cli_params.prometheus_host,
				port: cli_params.prometheus_port,
			})
			.into()
		} else {
			None.into()
		}
	}
}
