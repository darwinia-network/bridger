use crate::{
    api::{Darwinia, Shadow},
    error::Result,
    Config,
    service::GuardService,
};
use std::sync::Arc;
use actix::Actor;
use crate::service::ExtrinsicsService;

/// Run guard
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // apis
    let config = Config::new(&Config::default_data_dir()?)?; // TODO: add --data-dir
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia =  Arc::new(Darwinia::new(&config).await?);

    // extrinsic sender
    let extrinsics_service = ExtrinsicsService::new(darwinia.clone(), "".to_string(), dirs::home_dir().unwrap()).start();

    info!("Init API succeed!");

    // guard service
    let is_tech_comm_member = darwinia.sender.is_tech_comm_member().await?;
    let _guard_service =
        GuardService::new(shadow.clone(), darwinia.clone(), config.step.guard, is_tech_comm_member, extrinsics_service.recipient()).map(|g| {
            g.start()
        });

    log::info!("Ctrl-C to shut down");
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Ctrl-C received, shutting down");
    Ok(())
}
