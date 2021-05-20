use chain_relay::types::transfer::{BridgeName, ChainInfo, HexLaneId};
use getset::Getters;
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "substrate-relay", about = "Substrate relay")]
pub enum Opt {
	// /// Init substrate to substrate bridge
	// #[deprecated]
	// InitBridge {
	// 	/// bridge info
	// 	#[structopt(flatten)]
	// 	bridge_info: OptBridgeInfo,
	// },
	InitBridge {
		/// bridge info
		#[structopt(flatten)]
		bridge: RelayBridgeInfo,
	},
	/// Relay headers and messages
	Relay(OptRelayV2),
	// /// Start substrate relay
	// Start {
	// 	/// The config file path
	// 	#[structopt(short, long, parse(from_os_str))]
	// 	config: Option<PathBuf>,
	// 	/// Listen host, Default:  127.0.0.1
	// 	#[structopt(short, long)]
	// 	host: Option<String>,
	// 	/// Listen port, Default: 7890
	// 	#[structopt(short, long)]
	// 	port: Option<u32>,
	// 	/// Is enable authorization for request this server
	// 	#[structopt(long)]
	// 	enable_auth: bool,
	// },
	// /// Substrate relay config
	// Config(OptConfig),
}

#[derive(Debug, StructOpt, Getters)]
#[getset(get = "pub")]
#[deprecated]
pub struct OptConfig {
	/// Enable debug model, show more message
	#[structopt(long, long)]
	debug: bool,
	/// The server host by substrate-relay service
	#[structopt(long, default_value = "http://127.0.0.1:7890")]
	server: String,
	/// The token of substrate-relay service
	#[structopt(short = "k", long)]
	token: Option<String>,
	/// Config subordinate command
	#[structopt(subcommand)]
	sub_command: OptConfigSubcommand,
}

#[derive(Debug, StructOpt)]
#[deprecated]
pub enum OptConfigSubcommand {
	/// Config chain information
	Chain(OptChainCommand),
	/// Config service token
	Token(OptTokenCommand),
}

#[derive(Debug, StructOpt)]
#[deprecated]
pub enum OptChainCommand {
	/// List all chain
	List,
	/// Add a new chain
	Add {
		/// sub command
		#[structopt(flatten)]
		chain_info: OptChainInfo,
	},
	/// Update an exists chain
	Update {
		/// sub command
		#[structopt(flatten)]
		chain_info: OptChainInfo,
	},
	/// Remove an exists chain
	Remove {
		/// Chain name
		name: String,
	},
}

#[derive(Debug, StructOpt)]
#[deprecated]
pub enum OptTokenCommand {
	/// List all token
	List,
	/// Generate a new token
	Generate {
		/// Token remark
		#[structopt(short, long)]
		remark: Option<String>,
	},
	/// Remove a token
	Remove {
		/// Token value
		token: String,
	},
}

#[derive(Debug, Clone, StructOpt)]
#[deprecated]
pub struct OptChainInfo {
	/// Chain name
	pub name: String,
	/// Chain rpc host
	#[structopt(short, long)]
	host: String,
	/// Chain rpc port
	#[structopt(short, long)]
	port: u32,
	/// Chain signer
	#[structopt(short, long)]
	signer: String,
	/// Use secure websocket connection.
	#[structopt(long)]
	secure: bool,
	/// Chain signer password
	#[structopt(long)]
	signer_password: Option<String>,
}

#[derive(Debug, Clone, StructOpt)]
#[deprecated]
pub struct OptBridgeInfo {
	/// The server host by substrate-relay service
	#[structopt(long, default_value = "http://127.0.0.1:7890")]
	pub server: String,
	/// The token of substrate-relay service
	#[structopt(short = "k", long)]
	pub token: Option<String>,
	/// The source chain name of s2s bridge by configured
	#[structopt(short, long)]
	pub source: String,
	/// The target chain name of s2s bridge by configured
	#[structopt(short, long)]
	pub target: String,
}

#[derive(Debug, StructOpt)]
#[deprecated]
pub enum OptRelay {
	/// Start relay
	Start {
		/// bridge info
		#[structopt(flatten)]
		bridge_info: OptBridgeInfo,
		/// Hex-encoded lane id that should be served by the relay. Defaults to `00000000`.
		#[structopt(long, default_value = "00000000")]
		lane: Vec<String>,
		#[structopt(flatten)]
		prometheus_params: PrometheusParams,
	},
	/// Stop relay
	Stop {
		/// bridge info
		#[structopt(flatten)]
		bridge_info: OptBridgeInfo,
	},
	/// Relay status
	Status {
		/// bridge info
		#[structopt(flatten)]
		bridge_info: OptBridgeInfo,
	},
}

#[derive(Debug, StructOpt)]
pub enum OptRelayV2 {}

#[derive(Debug, Clone, StructOpt)]
pub struct RelayBridgeInfo {
	bridge: BridgeName,
	#[structopt(short, long)]
	source: String,
	#[structopt(short, long)]
	target: String,
	#[structopt(long)]
	target_signer: String,
	#[structopt(long)]
	target_signer_password: Option<String>,
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
	pub fn source_chain_info(&self) -> crate::error::Result<ChainInfo> {
		Ok(ChainInfo::new(self.source.clone(), None, None)?)
	}

	pub fn target_chain_info(&self) -> crate::error::Result<ChainInfo> {
		Ok(ChainInfo::new(
			self.target.clone(),
			Some(self.target_signer.clone()),
			self.target_signer_password.clone(),
		)?)
	}
}
