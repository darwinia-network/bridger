use crate::{api::Darwinia, error::Result, Config};
use std::sync::Arc;

/// Ecdsa
pub async fn exec(message: String) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Config::new(&Config::default_data_dir()?)?; // TODO: add --data-dir
	let darwinia = Arc::new(Darwinia::new(&config).await?);

	info!("Init API succeed!");

	let message = hex::decode(&message[2..])?;
	let mut buffer = [0u8; 32];
	buffer.copy_from_slice(&message);
	darwinia
		.ecdsa_sign_and_submit_signed_authorities(buffer)
		.await?;

	Ok(())
}
