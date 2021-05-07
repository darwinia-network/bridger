use structopt::StructOpt;

use opt::Opt;

use crate::error::Result;

mod opt;

pub async fn exec() -> Result<()> {
	let opt = Opt::from_args();
	match opt {
		Opt::InitBridge { source, target } => {
			debug!("init: {} -> {}", source, target);
		}
		Opt::Start { config, port } => {
			debug!(
				"TODO: Start substrate relay, config: {:?}, port: {:?}",
				config, port
			)
		}
		Opt::Config(config) => {
			debug!("TODO: config file path: {:?}", config);
		}
	}
	Ok(())
}
