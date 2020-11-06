use crate::api::{Darwinia, Shadow};
use crate::{
    // listener::Listener,
    result::{Error, Result},
    Config,
};
// use async_macros::select;
// use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use web3::{
    transports::http::Http,
    Web3,
};
use actix::Actor;

use crate::service::EthereumService;
use crate::service::RelayService;
use crate::service::RedeemService;
use crate::service::GuardService;
// use crate::service::SubscribeService;

/// Run the bridger
pub async fn exec(path: Option<PathBuf>) -> Result<()> {

    // --- Load config ---
    let config = Config::new(path.clone())?;
    if config.eth.rpc.starts_with("ws") {
        return Err(Error::Bridger(
            "Bridger currently doesn't support ethereum websocket transport".to_string(),
        ));
    }
    // print info
    info!("🔗 Connect to");
    info!("      Darwinia: {}", config.node);
    info!("        Shadow: {}", config.shadow);
    info!("      Ethereum: {}", config.eth.rpc);


    // --- Init APIs ---
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia = Arc::new(Darwinia::new(&config).await?);
    let web3 = Web3::new(Http::new(&config.eth.rpc).unwrap());
    // print info
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
    info!("🌱 Relay from ethereum block: {}", config.eth.start);

    // --- Start service with killer patch ---
    start_services(&config, &shadow, &darwinia, &web3).await
    // let killer = darwinia.client.rpc.client.killer.clone();
    // let never_exit = async {
    //     start_services(&config, &shadow, &darwinia, &web3).await;
    //     Ok::<(), Error>(())
    // };
    // let exit_on_ws_close = async {
    //     loop {
    //         if killer.lock().await.next().await.is_some() {
    //             return Err(Error::Bridger("WS Closed".into()));
    //         }
    //     }
    // };

    // select!(never_exit, exit_on_ws_close).await
}

async fn start_services(config: &Config, shadow: &Arc<Shadow>, darwinia: &Arc<Darwinia>, web3: &Web3<Http>) -> Result<()> {
    // ethereum service
    let contracts = EthereumService::parse_contract(config);
    let filters = EthereumService::parse_filter(config)?;
    let _ethereum_service = EthereumService::new(web3.clone(), contracts, filters, config.eth.start, config.step.ethereum).start();

    // relay service
    let last_confirmed = darwinia.last_confirmed().await.unwrap();
    let _relay_service = RelayService::new(shadow.clone(), darwinia.clone(), last_confirmed).start();

    // redeem service
    let _redeem_service = RedeemService::new(shadow.clone(), darwinia.clone(), config.step.redeem).start();

    // guard service
    GuardService::new(shadow.clone(), darwinia.clone(), config.step.guard).start();

    Ok(())
}
