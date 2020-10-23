use crate::{listener::Listener, result::Result, Config};
use std::path::PathBuf;

/// Run the bridger
pub async fn exec(path: Option<PathBuf>, verbose: bool) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_bridger");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let mut listener = Listener::from_config(Config::new(path)?).await?;
    listener.start().await?;
    Ok(())
}
