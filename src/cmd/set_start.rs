use crate::result::Result;
use std::path::PathBuf;
use crate::service::EthereumService;
use crate::Config;

/// set ethereum start
pub async fn exec(data_dir: Option<PathBuf>, start: u64) -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let data_dir = data_dir.unwrap_or(Config::default_data_dir()?);
    EthereumService::set_ethereum_start(data_dir, start).await?;
    println!("OK");

    Ok(())
}
