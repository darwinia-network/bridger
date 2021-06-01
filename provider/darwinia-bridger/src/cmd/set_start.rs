use crate::error::Result;
use crate::{tools, Settings};
use std::path::PathBuf;

/// set ethereum start
pub async fn exec(data_dir: Option<PathBuf>, start: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	let data_dir = data_dir.unwrap_or(Settings::default_data_dir()?);
	tools::set_cache(data_dir, tools::LAST_REDEEMED_CACHE_FILE_NAME, start).await?;
	println!("OK");

	Ok(())
}
