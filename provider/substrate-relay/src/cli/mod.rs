use structopt::StructOpt;

use opt::Opt;

use crate::error::Result;

mod config;
mod opt;
mod service;

pub async fn exec() -> Result<()> {
	let opt = Opt::from_args();
	match opt {
		Opt::InitBridge {
			server,
			token,
			source,
			target,
		} => {
			debug!("init: {} -> {}", source, target);
		}
		Opt::Start {
			config,
			host,
			port,
			enable_auth,
		} => {
			return service::exec(config, host, port, enable_auth).await;
		}
		Opt::Config(config) => {
			return config::exec(config).await;
		}
	}
	Ok(())
}
