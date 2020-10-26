use crate::{
    api::{Darwinia, Shadow},
    result::Result,
    Config,
    service::{Service, GuardService},
    pool::Pool,
};
use std::sync::{Arc, Mutex};

/// Run guard
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // apis
    let config = Config::new(None)?;
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia =  Arc::new(Darwinia::new(&config).await?);
    info!("Init API succeed!");

    // service
    let mut guard = GuardService::new(&config, shadow, darwinia.clone());

    // run guard
    let pool = Arc::new(Mutex::new(Pool::default()));
    if let Err(err) = guard.run(pool).await {
        error!("{:?}", err);
    }

    Ok(())
}
