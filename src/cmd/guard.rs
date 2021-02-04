use crate::service::ExtrinsicsService;
use crate::{
	api::Shadow,
	error::Result,
	service::GuardService,
	Config,
};
use actix::Actor;
use std::sync::Arc;
use darwinia::{
    Darwinia,
    Ethereum2Darwinia,
    DarwiniaAccount,
    FromEthereumAccount,
};

/// Run guard
pub async fn exec() -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Config::new(&Config::default_data_dir()?)?; // TODO: add --data-dir
	let shadow = Arc::new(Shadow::new(&config));
	let darwinia = Darwinia::new(&config.node).await?;
    let ethereum2darwinia = Ethereum2Darwinia::new(darwinia);
    let from_ethereum_account = FromEthereumAccount::new(
        DarwiniaAccount::new(
            config.seed.clone(),
            config.proxy.clone().map(|proxy| proxy.real[2..].to_string()),
        )
    );
	// extrinsic sender
	let extrinsics_service = ExtrinsicsService::new(
        Some(ethereum2darwinia.clone()), 
        None,
        Some(from_ethereum_account.clone()),
        None,
        "".to_string(), 
        dirs::home_dir().unwrap()).start();

	info!("Init API succeed!");

	// guard service
	let is_tech_comm_member = ethereum2darwinia.is_tech_comm_member(&from_ethereum_account).await?;
	let _guard_service = GuardService::new(
		shadow.clone(),
		ethereum2darwinia.clone(),
        from_ethereum_account.clone(),
		config.step.guard,
		is_tech_comm_member,
		extrinsics_service.recipient(),
	)
	.map(|g| g.start());

	log::info!("Ctrl-C to shut down");
	tokio::signal::ctrl_c().await.unwrap();
	log::info!("Ctrl-C received, shutting down");
	Ok(())
}
