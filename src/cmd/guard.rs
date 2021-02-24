use crate::service::ExtrinsicsService;
use crate::{
	api::{darwinia_api, Shadow},
	error::Result,
	service::GuardService,
	Settings,
};
use actix::Actor;
use rpassword::prompt_password_stdout;
use std::sync::Arc;

/// Run guard
pub async fn exec() -> Result<()> {
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
	let from_ethereum_account =
		darwinia_api::get_e2d_account(darwinia_api::get_darwinia_account(&config));
	// extrinsic sender
	let extrinsics_service = ExtrinsicsService::new(
		Some(ethereum2darwinia.clone()),
		None,
		Some(from_ethereum_account.clone()),
		None,
		"".to_string(),
		dirs::home_dir().unwrap(),
	)
	.start();

	info!("Init API succeed!");

	// guard service
	let is_tech_comm_member = ethereum2darwinia
		.is_tech_comm_member(None, &from_ethereum_account)
		.await?;
	let _guard_service = GuardService::new(
		shadow.clone(),
		ethereum2darwinia.clone(),
		from_ethereum_account.clone(),
		config.services.guard.step,
		is_tech_comm_member,
		extrinsics_service.recipient(),
	)
	.map(|g| g.start());

	log::info!("Ctrl-C to shut down");
	tokio::signal::ctrl_c().await.unwrap();
	log::info!("Ctrl-C received, shutting down");
	Ok(())
}
