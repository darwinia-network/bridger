use crate::{listener::Listener, result::{Result, Error}, Config};
use std::path::PathBuf;
use crate::{
    api::{Darwinia, Shadow},
    service::{EthereumService, GuardService, RedeemService, RelayService},
};
use std::sync::Arc;
use web3::transports::http::Http;

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

    // config
    let config = Config::new(path)?;
    if config.eth.rpc.starts_with("ws") {
        return Err(Error::Bridger(
            "Bridger currently doesn't support ethereum websocket transport".to_string(),
        ));
    }

    // APIs
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia = Arc::new(Darwinia::new(&config).await?);

    // Services
    let ethereum = <EthereumService<Http>>::new_http(&config)?;
    let relay = RelayService::new(&config, shadow.clone(), darwinia.clone());
    let redeem = RedeemService::new(&config, shadow.clone(), darwinia.clone());
    let guard = GuardService::new(&config, shadow, darwinia.clone());

    // Startup infomations
    info!("🔗 Connect to");
    info!("      Darwinia: {}", config.node);
    info!("        Shadow: {}", config.shadow);
    info!("      Ethereum: {}", config.eth.rpc);
    let account_id = &darwinia.account.account_id;
    let roles = darwinia.account.role_names().await?;
    match &darwinia.account.real {
        None => {
            info!("🧔 Relayer({:?}): 0x{:?}", roles, account_id);
        },
        Some(real_account_id) => {
            info!("🧔 Proxy Relayer: 0x{:?}", account_id);
            info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);

        }
    }
    info!("🌱 Relay from ethereum block: {}", config.eth.start);

    // listeners
    let mut listener = Listener::default();
    listener.register(ethereum)?;
    listener.register(relay)?;
    listener.register(redeem)?;
    if let Err(err) = guard.role_checking().await {
        warn!("{}", err.to_string());
    } else {
        listener.register(guard)?;
    }
    listener.start(config.eth.start).await?;

    Ok(())
}
