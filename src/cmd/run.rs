use crate::{
    api::{Darwinia, Shadow},
    service::{EthereumService, GuardService, RedeemService, RelayService, SubscribeService},
};
use crate::{
    listener::Listener,
    result::{Error, Result},
    Config,
};
use async_macros::select;
use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use web3::transports::http::Http;

/// Run the bridger
pub async fn exec(path: Option<PathBuf>) -> Result<()> {
    info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

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
    let killer = &darwinia.client.rpc.client.killer;

    // Services
    let ethereum = <EthereumService<Http>>::new_http(&config)?;
    let relay = RelayService::new(&config, shadow.clone(), darwinia.clone());
    let redeem = RedeemService::new(&config, shadow.clone(), darwinia.clone());
    let guard = GuardService::new(&config, shadow.clone(), darwinia.clone());
    let subscribe = SubscribeService::new(shadow, darwinia.clone());

    // Startup infomations
    info!("🔗 Connect to");
    info!("      Darwinia: {}", &config.node);
    info!("        Shadow: {}", &config.shadow);
    info!("      Ethereum: {}", &config.eth.rpc);
    let account_id = &darwinia.account.account_id;
    let roles = darwinia.account.role_names().await?;
    match &darwinia.account.real {
        None => {
            info!("🧔 Relayer({:?}): 0x{:?}", roles, account_id);
        }
        Some(real_account_id) => {
            info!("🧔 Proxy Relayer: 0x{:?}", account_id);
            info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
        }
    }
    info!("🌱 Relay from ethereum block: {}", &config.eth.start);

    // listeners
    let mut listener = Listener::default();
    listener.register(ethereum)?;
    listener.register(relay)?;
    listener.register(redeem)?;
    listener.register(subscribe)?;
    if let Err(err) = guard.role_checking().await {
        warn!("{}", err.to_string());
    } else {
        listener.register(guard)?;
    }

    let never_exit = async {
        listener.start(config.eth.start).await?;

        Ok::<(), Error>(())
    };
    let exit_on_ws_close = async {
        loop {
            if killer.lock().await.next().await.is_some() {
                return Err(Error::Bridger("WS Closed".into()));
            }
        }
    };

    select!(never_exit, exit_on_ws_close).await
}
