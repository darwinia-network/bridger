use std::path::PathBuf;

use getset::Getters;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "substrate-relay", about = "Substrate relay")]
pub enum Opt {
	/// Init substrate to substrate bridge
	InitBridge {
		/// The server host by substrate-relay service
		#[structopt(long, default_value = "http://127.0.0.1:7890")]
		server: String,
		/// The token of substrate-relay service
		#[structopt(short = "k", long)]
		token: Option<String>,
		/// The source chain name of s2s bridge by configured
		#[structopt(short, long)]
		source: String,
		/// The target chain name of s2s bridge by configured
		#[structopt(short, long)]
		target: String,
	},
	/// Start substrate relay
	Start {
		/// The config file path
		#[structopt(short, long, parse(from_os_str))]
		config: Option<PathBuf>,
		/// Listen host, Default:  127.0.0.1
		#[structopt(short, long)]
		host: Option<String>,
		/// Listen port, Default: 7890
		#[structopt(short, long)]
		port: Option<u32>,
		/// Is enable authorization for request this server
		#[structopt(long)]
		enable_auth: bool,
	},
	/// Substrate relay config
	Config(OptConfig),
}

#[derive(Debug, StructOpt, Getters)]
#[getset(get = "pub")]
pub struct OptConfig {
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
pub enum OptConfigSubcommand {
	/// Config chain information
	Chain(OptChainCommand),
	/// Config service token
	Token(OptTokenCommand),
}

#[derive(Debug, StructOpt)]
pub enum OptChainCommand {
	/// List all chain
	List,
	/// Add a new chain
	Add {
		/// Chain name
		name: String,
		/// Chain rpc host
		#[structopt(short, long)]
		host: String,
		/// Chain rpc port
		#[structopt(short, long)]
		port: u32,
		/// Chain signer
		#[structopt(short, long)]
		signer: String,
	},
	/// Update an exists chain
	Update {
		/// Chain name
		name: String,
		/// Chain rpc host
		#[structopt(short, long)]
		host: String,
		/// Chain rpc port
		#[structopt(short, long)]
		port: u32,
		/// Chain signer
		#[structopt(short, long)]
		signer: String,
	},
	/// Remove an exists chain
	Remove {
		/// Chain name
		name: String,
	},
}

#[derive(Debug, StructOpt)]
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
