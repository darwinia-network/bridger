use crate::api::{darwinia_api, Shadow};
use crate::{
	// listener::Listener,
	error::{Error, Result},
	Settings,
};
use std::path::PathBuf;
use crate::service::heco::HecoLogsHandler;
use web3::Web3;
use web3::transports::Http;

/// Run the heco bridger
pub async fn exec(data_dir: Option<PathBuf>, verbose: bool) -> Result<()> {
	if std::env::var("RUST_LOG").is_err() {
		if verbose {
			std::env::set_var("RUST_LOG", "info,darwinia_bridger");
		} else {
			std::env::set_var("RUST_LOG", "info");
		}
	}
	env_logger::init();

	// --- Data dir ---
	let data_dir = data_dir.unwrap_or_else(|| Settings::default_data_dir().unwrap());
	// --- Load config ---
	let mut config = Settings::new_heco(&data_dir)?;

	let web3 = Web3::new(
		Http::new(&config.ethereum.rpc)
			.map_err(|e| Error::NewHttpError(config.ethereum.rpc.clone(), e.to_string()))?,
	);
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;

	let mut tracker = HecoLogsHandler::new(
		config.clone(),
		web3.clone(),
		data_dir.clone(),
		4006177,
		darwinia.clone(),
	);

	tracker.start().await;
	Ok(())
}
