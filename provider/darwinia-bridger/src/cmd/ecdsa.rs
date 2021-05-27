use crate::{api::darwinia_api, error::Error, error::Result, Settings};
use rpassword::prompt_password_stdout;

/// Ecdsa
pub async fn exec(message: String) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let darwinia2ethereum = darwinia_api::get_d2e_instance(darwinia.clone());
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let to_ethereum_account = darwinia_api::get_d2e_account(darwinia_account, &config);

	info!("Init API succeed!");

	let message = array_bytes::hex2bytes(&message[2..])
		.map_err(|_| Error::Hex2Bytes("message[2..]".into()))?;
	let mut buffer = [0u8; 32];
	buffer.copy_from_slice(&message);
	darwinia2ethereum
		.ecdsa_sign_and_submit_signed_authorities(&to_ethereum_account, buffer)
		.await?;

	Ok(())
}
