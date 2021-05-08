use structopt::StructOpt;

use opt::Opt;

use crate::error::Result;

mod opt;
mod service;

pub async fn exec() -> Result<()> {
	let opt = Opt::from_args();
	match opt {
		Opt::InitBridge { source, target } => {
			debug!("init: {} -> {}", source, target);
		}
		Opt::Start { config, host, port } => {
			return service::exec(config, host, port).await;
		}
		Opt::Config(config) => {
			debug!("TODO: config file path: {:?}", config);
		}
	}
	Ok(())
}
