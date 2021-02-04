use crate::service::ExtrinsicsService;
use crate::{
	api::Shadow,
	error::Result,
	service::RelayService,
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

/// Affirm
pub async fn exec(block: u64) -> Result<()> {
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
        Some(from_ethereum_account),
        None,
        "".to_string(), 
        dirs::home_dir().unwrap()).start();

	info!("Init API succeed!");

	// affirm
	if let Err(err) =
		RelayService::affirm(
            ethereum2darwinia, 
            shadow, 
            block, 
            extrinsics_service.recipient()).await
	{
		error!("{:?}", err);
	}

	Ok(())
}
