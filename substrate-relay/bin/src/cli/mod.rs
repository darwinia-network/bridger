use structopt::StructOpt;

use types::Opt;

use crate::error::Result;

mod config;
mod initialize;
mod relay;
mod server;
mod types;

pub async fn exec() -> Result<()> {
	let opt = Opt::from_args();
	match opt {
		Opt::InitBridge { bridge_info } => {
			return initialize::exec(bridge_info).await;
		}
		Opt::Relay(opt_relay) => {
			return relay::exec(opt_relay).await;
		}
		Opt::Start {
			config,
			host,
			port,
			enable_auth,
		} => {
			return server::exec(config, host, port, enable_auth).await;
		}
		Opt::Config(config) => {
			return config::exec(config).await;
		}
	}
}
