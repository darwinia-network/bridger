use crate::error::Result;
use std::path::PathBuf;
use crate::service::RedeemService;
use crate::Config;

/// set ethereum start
pub async fn exec(data_dir: Option<PathBuf>, start: u64) -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let data_dir = data_dir.unwrap_or(Config::default_data_dir()?);
    RedeemService::set_last_redeemed(data_dir, start).await?;
    println!("OK");

    Ok(())
}
