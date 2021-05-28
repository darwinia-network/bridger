use chain_relay::types::transfer::{BridgeName, ChainInfo, HexLaneId, PrometheusParamsInfo};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "substrate-relay", about = "Substrate relay")]
pub enum Opt {
	/// Init bridge
	InitBridge {
		/// bridge info
		#[structopt(flatten)]
		bridge: RelayBridgeInfo,
	},
	/// Relay headers and messages
	Relay {
		/// bridge info
		#[structopt(flatten)]
		bridge: RelayBridgeInfo,
		#[structopt(long, default_value = "00000000")]
		lanes: Vec<HexLaneId>,
		#[structopt(flatten)]
		prometheus: PrometheusParams,
	},
}

#[derive(Debug, Clone, StructOpt)]
pub struct RelayBridgeInfo {
	/// The bridge name
	pub bridge: BridgeName,
	#[structopt(short, long)]
	pub source: String,
	#[structopt(short, long)]
	pub target: String,
	#[structopt(long)]
	pub source_signer: Option<String>,
	#[structopt(long)]
	pub source_signer_password: Option<String>,
	#[structopt(long)]
	pub target_signer: Option<String>,
	#[structopt(long)]
	pub target_signer_password: Option<String>,
}

/// Prometheus metrics params.
#[derive(Debug, Clone, StructOpt)]
pub struct PrometheusParams {
	/// Do not expose a Prometheus metric endpoint.
	#[structopt(long)]
	pub no_prometheus: bool,
	/// Expose Prometheus endpoint at given interface.
	#[structopt(long, default_value = "127.0.0.1")]
	pub prometheus_host: String,
	/// Expose Prometheus endpoint at given port.
	#[structopt(long, default_value = "9616")]
	pub prometheus_port: u16,
}

impl RelayBridgeInfo {
	pub fn source_chain_info(&self) -> anyhow::Result<ChainInfo> {
		ChainInfo::new(
			self.source.clone(),
			self.source_signer.clone(),
			self.source_signer_password.clone(),
		)
	}

	pub fn target_chain_info(&self) -> anyhow::Result<ChainInfo> {
		ChainInfo::new(
			self.target.clone(),
			self.target_signer.clone(),
			self.target_signer_password.clone(),
		)
	}
}

impl PrometheusParams {
	pub fn prometheus_info(&self) -> PrometheusParamsInfo {
		let mut prometheus_info = PrometheusParamsInfo::default();
		prometheus_info.set_no_prometheus(self.no_prometheus);
		prometheus_info.set_prometheus_host(self.prometheus_host.clone());
		prometheus_info.set_prometheus_port(self.prometheus_port);
		prometheus_info
	}
}
