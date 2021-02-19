use crate::{api::Darwinia, error::Result, Settings};
use rpassword::prompt_password_stdout;
use std::sync::Arc;

/// Affirm
pub async fn exec(json: String) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let darwinia = Arc::new(Darwinia::new(&config).await?);

	// build from json string
	let parcel: primitives::chain::ethereum::EthereumRelayHeaderParcel =
		serde_json::from_str(&json).unwrap();

	// affirm
	let hash = darwinia.affirm(parcel).await.unwrap();
	println!("Extrinsic hash: {:?}", hash);

	Ok(())
}
