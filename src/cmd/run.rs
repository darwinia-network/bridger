use crate::{
    api::{Darwinia, Shadow},
    service::{EthereumService, GuardService, RedeemService, RelayService, SubscribeService},
};
use crate::{
    listener::Listener,
    result::{Error, Result},
    Config,
};
use std::path::PathBuf;
use std::sync::Arc;
use substrate_subxt::sp_core::Pair;
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
    let guard = GuardService::new(&config, shadow.clone(), darwinia.clone());
    let subscribe = SubscribeService::new(shadow.clone(), darwinia.clone());

    // Startup infomations
    info!("🔗 Connect to");
    info!("      Darwinia: {}", config.node);
    info!("        Shadow: {}", config.shadow);
    info!("      Ethereum: {}", config.eth.rpc);
    let signer_public = &darwinia.signer.signer().public();
    match &config.proxy {
        None => {
            info!("🧔 Relayer({:?}): 0x{:?}", darwinia.role, signer_public);
        }
        Some(proxy) => {
            info!(
                "🧔 Proxy Relayer({:?}): 0x{:?}",
                darwinia.role, signer_public
            );
            info!("👴 Real Account: {}", proxy.real);
        }
    }
    info!("🌱 Relay from ethereum block: {}", config.eth.start);

    let mut listener = Listener::default();

    listener.register(ethereum)?;
    listener.register(relay)?;
    listener.register(redeem)?;
    listener.register(subscribe)?;
    if let Err(err) = guard.role_checking() {
        warn!("{}", err.to_string());
    } else {
        listener.register(guard)?;
    }

    listener.start().await?;
    Ok(())
}
