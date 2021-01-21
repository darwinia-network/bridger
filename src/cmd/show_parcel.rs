use crate::{api::Shadow, error::Result, Settings};
use std::sync::Arc;

/// Affirm a faked affirmation
pub async fn exec(block: u64, json: bool) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	let shadow = Arc::new(Shadow::new(&config));

	// Get parcel
	let parcel = shadow.parcel(block as usize).await?;

	// print
	if json {
		println!("{}", serde_json::to_string(&parcel)?);
	} else {
		println!("{}", parcel);
	}

	Ok(())
}
