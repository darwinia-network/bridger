use crate::service::ExtrinsicsService;
use crate::{
	api::{Darwinia, Shadow},
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
	let darwinia = Arc::new(Darwinia::new(&config).await?);

	// extrinsic sender
	let extrinsics_service =
		ExtrinsicsService::new(darwinia.clone(), "".to_string(), dirs::home_dir().unwrap()).start();

	info!("Init API succeed!");

	// guard service
	let is_tech_comm_member = darwinia.sender.is_tech_comm_member(None).await?;
	let _guard_service = GuardService::new(
		shadow.clone(),
		darwinia.clone(),
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
