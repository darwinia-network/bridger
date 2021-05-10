use std::env;
use std::path::PathBuf;

use crate::error::Result;
use crate::persist::{Generic, Persist};
use crate::server::webserver::*;

fn path_config_file(config: Option<PathBuf>) -> Result<PathBuf> {
	let mut dir = env::current_exe()?;
	Ok(config.unwrap_or_else(|| {
		dir.pop();
		dir.push("config.toml");
		dir
	}))
}

pub async fn exec(
	config: Option<PathBuf>,
	host: Option<String>,
	port: Option<u32>,
	enable_auth: bool,
) -> Result<()> {
	let config_file = path_config_file(config.clone())?;
	info!("Use config: {}", config_file.display());
	let mut persist = Persist::load_from_file(config_file).await?;
	let generic: &mut Generic = persist.generic_mut();

	if let Some(h) = host {
		generic.set_host(h);
	}
	if let Some(p) = port {
		generic.set_port(p);
	}
	generic.set_enable_auth(enable_auth);
	persist.store().await?;
	let server = WebServer::builder().persist(persist).build();
	server.run().await
}
