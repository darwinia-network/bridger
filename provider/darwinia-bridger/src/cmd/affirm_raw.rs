use crate::{api::darwinia_api, error::Result, Settings};
use rpassword::prompt_password_stdout;

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
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia);
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let e2d_account = darwinia_api::get_e2d_account(darwinia_account);

	// build from json string
	let parcel: primitives::chain::ethereum::EthereumRelayHeaderParcel =
		serde_json::from_str(&json).unwrap();

	// affirm
	let hash = ethereum2darwinia
		.affirm(&e2d_account, parcel)
		.await
		.unwrap();
	println!("Extrinsic hash: {:?}", hash);

	Ok(())
}
