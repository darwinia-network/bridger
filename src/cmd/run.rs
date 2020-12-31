use crate::api::{Darwinia, Shadow};
use crate::{
	// listener::Listener,
	error::{Error, Result},
	Config,
};
use actix::Actor;
use async_macros::select;
use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use web3::{transports::http::Http, Web3};

use crate::api::Ethereum;
use crate::error::BizError;
use crate::service::EthereumService;
use crate::service::ExtrinsicsService;
use crate::service::GuardService;
use crate::service::MsgStop;
use crate::service::RedeemService;
use crate::service::RelayService;
use crate::service::SubscribeService;
use crate::tools;

/// Run the bridger
pub async fn exec(data_dir: Option<PathBuf>, verbose: bool) {
	if std::env::var("RUST_LOG").is_err() {
		if verbose {
			std::env::set_var("RUST_LOG", "info,darwinia_bridger");
		} else {
			std::env::set_var("RUST_LOG", "info");
		}
	}
	env_logger::init();

	while let Err(e) = run(data_dir.clone()).await {
		if let Some(Error::NoEthereumStart) = e.downcast_ref() {
			error!("{:?}", e);
			break;
		} else if let Some(Error::NoDarwiniaStart) = e.downcast_ref() {
			error!("{:?}", e);
			break;
		} else {
			error!("{:?}", e);
			info!("Bridger will restart in 30 seconds...");
			time::delay_for(Duration::from_secs(30)).await;
		}
	}
}

async fn run(data_dir: Option<PathBuf>) -> Result<()> {
	info!(
		"✌️  {} v{}",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION")
	);

	// --- Data dir ---
	let data_dir = data_dir.unwrap_or(Config::default_data_dir()?);
	info!("💾 Data dir: {}", data_dir.to_str().unwrap());

	// --- Load config ---
	let config = Config::new(&data_dir)?;
	if config.eth.rpc.starts_with("ws") {
		return Err(BizError::Bridger(
			"Bridger currently doesn't support ethereum websocket transport".to_string(),
		)
		.into());
	}

	// --- Init APIs ---
	let shadow = Arc::new(Shadow::new(&config));
	let darwinia = Arc::new(Darwinia::new(&config).await?);
	let web3 = Web3::new(Http::new(&config.eth.rpc).unwrap());

	// --- Network ---
	let runtime_version: sp_version::RuntimeVersion =
		darwinia.client.rpc.runtime_version(None).await?;
	let spec_name = runtime_version.spec_name.to_string();

	// --- Print startup info ---
	info!("🔗 Connect to");
	info!("   Darwinia {}: {}", &spec_name, config.node);
	info!("   Shadow: {}", config.shadow);
	info!("   Ethereum: {}", config.eth.rpc);
	let account_id = &darwinia.sender.account_id;
	let roles = darwinia.sender.role_names().await?;
	match &darwinia.sender.real {
		None => {
			info!("🧔 Relayer({:?}): 0x{:?}", roles, account_id);
		}
		Some(real_account_id) => {
			info!("🧔 Proxy Relayer: 0x{:?}", account_id);
			info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
		}
	}

	// --- Start services ---
	start_services(&config, &shadow, &darwinia, &web3, data_dir, spec_name).await
}

async fn start_services(
	config: &Config,
	shadow: &Arc<Shadow>,
	darwinia: &Arc<Darwinia>,
	web3: &Web3<Http>,
	data_dir: PathBuf,
	spec_name: String,
) -> Result<()> {
	let last_redeemed = tools::get_cache(
		data_dir.clone(),
		tools::LAST_REDEEMED_CACHE_FILE_NAME,
		Error::NoEthereumStart,
	)
	.await?;
	info!("🌱 Relay from ethereum block: {}", last_redeemed + 1);

	let last_tracked_ethereum_block = tools::get_cache(
		data_dir.clone(),
		tools::LAST_TRACKED_ETHEREUM_BLOCK_FILE_NAME,
		Error::NoDarwiniaStart,
	)
	.await?;
	info!(
		"🌱 Scan darwinia from block: {}",
		last_tracked_ethereum_block + 1
	);

	// extrinsic sender
	let extrinsics_service =
		ExtrinsicsService::new(darwinia.clone(), spec_name.clone(), data_dir.clone()).start();

	// relay service
	let last_confirmed = darwinia.last_confirmed().await.unwrap();
	let relay_service = RelayService::new(
		shadow.clone(),
		darwinia.clone(),
		last_confirmed,
		config.step.relay,
		extrinsics_service.clone().recipient(),
	)
	.start();

	// redeem service
	let redeem_service = RedeemService::new(
		shadow.clone(),
		darwinia.clone(),
		config.step.redeem,
		extrinsics_service.clone().recipient(),
	)
	.start();

	// ethereum service
	let ethereum_service = EthereumService::new(
		config.clone(),
		web3.clone(),
		darwinia.clone(),
		last_redeemed + 1,
		relay_service.clone().recipient(),
		redeem_service.clone().recipient(),
		data_dir.clone(),
	)
	.start();

	// guard service
	let is_tech_comm_member = darwinia.sender.is_tech_comm_member().await?;
	let guard_service = GuardService::new(
		shadow.clone(),
		darwinia.clone(),
		config.step.guard,
		is_tech_comm_member,
		extrinsics_service.clone().recipient(),
	)
	.map(|g| g.start());

	//
	let ethereum = Ethereum::new(web3.clone(), &config.clone())?;
	let mut subscribe = SubscribeService::new(
		darwinia.clone(),
		ethereum,
		extrinsics_service.clone().recipient(),
		spec_name,
		(last_tracked_ethereum_block as u32) + 1,
		data_dir.clone(),
	);
	let b = async {
		if let Err(e) = subscribe.start().await {
			return Err(e);
		}
		Ok(())
	};

	let killer = darwinia.client.rpc.client.killer.clone();
	let c = async {
		loop {
			if killer.lock().await.next().await.is_some() {
				return Err(BizError::Bridger("Jsonrpsee's ws connection closed".into()).into());
			}
		}
	};

	if let Err(e) = select!(b, c).await {
		ethereum_service.do_send(MsgStop {});
		relay_service.do_send(MsgStop {});
		redeem_service.do_send(MsgStop {});
		if let Some(guard_service) = guard_service {
			guard_service.do_send(MsgStop {});
		}
		subscribe.stop();
		extrinsics_service.do_send(MsgStop {});
		Err(e)
	} else {
		Ok(())
	}
}
