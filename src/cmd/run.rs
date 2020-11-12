use crate::api::{Darwinia, Shadow};
use crate::{
    // listener::Listener,
    result::{Error, Result},
    Config,
};
use async_macros::select;
use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use web3::{
    transports::http::Http,
    Web3,
};
use actix::{Actor, System};
use substrate_subxt::sp_core::crypto::*;

use crate::service::EthereumService;
use crate::service::RelayService;
use crate::service::RedeemService;
use crate::service::GuardService;
use crate::service::SubscribeService;

/// Run the bridger
pub async fn exec(data_dir: Option<PathBuf>) -> Result<()> {
    log::info!("✌️  {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // --- Data dir
    let data_dir = data_dir.unwrap_or(Config::default_data_dir()?);
    info!("💾 Data dir: {}", data_dir.to_str().unwrap());

    // --- Load config ---
    let config = Config::new(&data_dir)?;
    if config.eth.rpc.starts_with("ws") {
        return Err(Error::Bridger(
            "Bridger currently doesn't support ethereum websocket transport".to_string(),
        ));
    }

    // --- Init APIs ---
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia = Arc::new(Darwinia::new(&config).await?);
    let web3 = Web3::new(Http::new(&config.eth.rpc).unwrap());

    // --- Network ---
    let runtime_version: sp_version::RuntimeVersion = darwinia.client.rpc.runtime_version(None).await?;
    let network = if runtime_version.spec_name.to_string() == "Crab" {
        "Crab"
    } else {
        set_default_ss58_version(Ss58AddressFormat::DarwiniaAccount);
        "Mainnet"
    };

    // --- Print startup info ---
    info!("🔗 Connect to");
    info!("   Darwinia {}: {}", network, config.node);
    info!("   Shadow: {}", config.shadow);
    info!("   Ethereum: {}", config.eth.rpc);
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

    // --- Start services ---
    let killer = darwinia.client.rpc.client.killer.clone();
    let never_exit = async {
        start_services(&config, &shadow, &darwinia, &web3, data_dir).await?;

        log::info!("Ctrl-C to shut down");
        tokio::signal::ctrl_c().await.unwrap();
        log::info!("Ctrl-C received, shutting down");
        System::current().stop();
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

async fn start_services(config: &Config, shadow: &Arc<Shadow>, darwinia: &Arc<Darwinia>, web3: &Web3<Http>, data_dir: PathBuf) -> Result<()> {
    // --- Read ethereum start from cache ---
    let ethereum_start = EthereumService::get_ethereum_start(&data_dir, &web3).await?;
    info!("🌱 Relay from ethereum block: {}", ethereum_start);

    // relay service
    let last_confirmed = darwinia.last_confirmed().await.unwrap();
    let relay_service = RelayService::new(shadow.clone(), darwinia.clone(), last_confirmed, config.step.relay).start();

    // redeem service
    let redeem_service = RedeemService::new(shadow.clone(), darwinia.clone(), config.step.redeem).start();

    // ethereum service
    EthereumService::new(
        config.clone(),
        web3.clone(),
        darwinia.clone(),
        relay_service.recipient(),
        redeem_service.recipient(),
        data_dir,
        ethereum_start,
    ).start();

    // guard service
    if let Ok(guard_service) = GuardService::new(shadow.clone(), darwinia.clone(), config.step.guard).await {
        guard_service.start();
    }

    // subscribe service
    let mut subscribe = SubscribeService::new(shadow.clone(), darwinia.clone()).await?;
    subscribe.start().await?;

    Ok(())
}
