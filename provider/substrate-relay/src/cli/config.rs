use crate::cli::opt::{OptConfig, OptConfigSubcommand};

use crate::error;

pub async fn exec(config: OptConfig) -> error::Result<()> {
	let server: &String = config.server();
	let token: &Option<String> = config.token();
	let sub: &OptConfigSubcommand = config.sub_command();
	debug!("{}: {:?} => {:?}", server, token, sub);
	Ok(())
}
