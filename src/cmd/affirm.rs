use crate::service::ExtrinsicsService;
use crate::{
	api::{darwinia_api, Shadow},
	error::Result,
	service::RelayService,
	Settings,
};
use actix::Actor;
use rpassword::prompt_password_stdout;
use std::sync::Arc;

/// Affirm
pub async fn exec(block: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let shadow = Arc::new(Shadow::new(&config));
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia);

	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let e2d_account = darwinia_api::get_e2d_account(darwinia_account);

	// extrinsic sender
	let extrinsics_service = ExtrinsicsService::new(
		Some(ethereum2darwinia.clone()),
		None,
		Some(e2d_account),
		None,
		"".to_string(),
		dirs::home_dir().unwrap(),
	)
	.start();

	info!("Init API succeed!");

	// affirm
	if let Err(err) = RelayService::affirm(
		ethereum2darwinia,
		shadow,
		block,
		extrinsics_service.recipient(),
	)
	.await
	{
		error!("{:?}", err);
	}

	Ok(())
}
