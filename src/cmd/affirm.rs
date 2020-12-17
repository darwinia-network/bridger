use crate::{
    api::{Darwinia, Shadow},
    error::Result,
    Config,
    service::RelayService,
};
use std::sync::Arc;
use crate::service::ExtrinsicsService;
use actix::Actor;

/// Affirm
pub async fn exec(block: u64) -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // apis
    let config = Config::new(&Config::default_data_dir()?)?; // TODO: add --data-dir
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia =  Arc::new(Darwinia::new(&config).await?);

    // extrinsic sender
    let extrinsics_service = ExtrinsicsService::new(darwinia.clone(), "".to_string(), dirs::home_dir().unwrap()).start();

    info!("Init API succeed!");

    // affirm
    if let Err(err) = RelayService::affirm(darwinia, shadow, block, extrinsics_service.recipient()).await {
        error!("{:?}", err);
    }

    Ok(())
}
