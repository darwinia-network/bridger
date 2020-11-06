use crate::{
    api::{Darwinia, Shadow},
    result::Result,
    Config,
    service::RelayService,
};
use std::sync::Arc;

/// Affirm
pub async fn exec(block: u64) -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    // apis
    let config = Config::new(None)?;
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia =  Arc::new(Darwinia::new(&config).await?);
    info!("Init API succeed!");

    // affirm
    if let Err(err) = RelayService::affirm(darwinia, shadow, block).await {
        error!("{:?}", err);
    }

    Ok(())
}
