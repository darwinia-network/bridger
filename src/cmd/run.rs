use crate::{listener::Listener, result::Result, Config};
use std::path::PathBuf;
use substrate_subxt::sp_runtime::app_crypto::Pair as PairTrait;
use sp_keyring::sr25519::sr25519::Pair;

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


    let config = Config::new(path)?;

    info!("⛓ Connect to");
    info!("      Darwinia: {}", config.node);
    info!("        Shadow: {}", config.shadow);
    info!("      Ethereum: {}", config.eth.rpc);
    let pair = Pair::from_string(&config.seed, None).unwrap();
    match &config.proxy {
        None => info!("🔨 Relayer: {:?}", pair.public()),
        Some(proxy) => {
            info!("🔨   Proxy: {:?}", pair.public());
            info!("🙌🔨  Real: {}", proxy.real);
        }
    }
    info!("Relay from block: {}", config.eth.start);

    let mut listener = Listener::from_config(config).await?;
    listener.start().await?;
    Ok(())
}
