use crate::{
    api::{Darwinia, Shadow},
    result::Result,
    Config,
    service::GuardService,
};
use std::sync::Arc;
use actix::{Actor, System};
use crate::result::Error::Bridger;
use std::time::Duration;

/// Run guard
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // apis
    let config = Config::new(None)?;
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia =  Arc::new(Darwinia::new(&config).await?);
    info!("Init API succeed!");

    // guard service
    let guard_service = GuardService::new(shadow, darwinia, config.step.guard).start();
    tokio::time::delay_for(Duration::from_secs(2)).await;
    if guard_service.connected() {
        log::info!("Ctrl-C to shut down");
        tokio::signal::ctrl_c().await.unwrap();
        log::info!("Ctrl-C received, shutting down");
        System::current().stop();
        Ok(())
    } else {
        Err(Bridger("Guard service is not running.".to_string()))
    }

}
