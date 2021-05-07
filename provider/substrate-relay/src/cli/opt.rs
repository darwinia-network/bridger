use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "substrate-relay", about = "Substrate relay")]
pub enum Opt {
	/// Init substrate to substrate bridge
	InitBridge {
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
		#[structopt(short, long)]
		config: Option<String>,
		/// Listen port
		#[structopt(short, long, default_value = "7890")]
		port: u32,
	},
	/// Substrate relay config
	Config(OptConfig),
}

#[derive(Debug, StructOpt)]
pub struct OptConfig {
	/// The host by substrate-relay service
	#[structopt(short, long, default_value = "http://127.0.0.1:7890")]
	host: String,
	/// The token of substrate-relay service
	#[structopt(short, long)]
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
		#[structopt(short, long)]
		name: String,
	},
	/// Update an exists chain
	Update {
		/// Chain name
		#[structopt(short, long)]
		name: String,
	},
	/// Remove an exists chain
	Remove {
		/// Chain name
		#[structopt(short, long)]
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
