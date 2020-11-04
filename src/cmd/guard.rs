use crate::{
    api::{Darwinia, Shadow},
    result::Result,
    Config,
    service::GuardService,
};
use std::sync::Arc;
use actix::{Actor, System};

/// Run guard
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // // apis
    // let config = Config::new(None)?;
    // let shadow = Arc::new(Shadow::new(&config));
    // let darwinia =  Arc::new(Darwinia::new(&config).await?);
    // info!("Init API succeed!");
    //
    // let system = System::new("guard");
    // let guard_service = GuardService::new(shadow, darwinia.clone(), config.step.guard).start();
    // system.run();
    //
    // let _ = my_relay_service.send(MsgStart{}).await;
    //
    // //
    // // tokio::signal::ctrl_c().await.unwrap();
    // // info!("Ctrl-C received, shutting down");
    // // System::current().stop();

    Ok(())
}
