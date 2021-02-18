use crate::api::{Darwinia, Shadow};
use crate::{
	// listener::Listener,
	error::{Error, Result},
	Settings,
};
use actix::Actor;
use async_macros::select;
use futures::StreamExt;
use rpassword::prompt_password_stdout;
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

	// --- Data dir ---
	let data_dir = data_dir.unwrap_or_else(|| Settings::default_data_dir().unwrap());
	// --- Load config ---
	let mut config = Settings::new(&data_dir).unwrap();

	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:").unwrap();
		if config.decrypt(&passwd).is_err() {
			return;
		}
	}

	while let Err(e) = run(data_dir.clone(), &config).await {
		if let Some(Error::NoEthereumStart) = e.downcast_ref() {
			error!("{:?}", e);
			break;
		} else if let Some(Error::NoDarwiniaStart) = e.downcast_ref() {
			error!("{:?}", e);
			break;
		} else if let Some(Error::NoAuthoritySignerSeed) = e.downcast_ref() {
			error!("{:?}", e);
			break;
		} else {
			error!("{:?}", e);
			info!("Bridger will restart in 30 seconds...");
			time::delay_for(Duration::from_secs(30)).await;
		}
	}
}

async fn run(data_dir: PathBuf, config: &Settings) -> Result<()> {
	info!(
		"✌️  {} v{}",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION")
	);

	info!("💾 Data dir: {}", data_dir.to_str().unwrap());
	if config.ethereum.rpc.starts_with("ws") {
		return Err(BizError::Bridger(
			"Bridger currently doesn't support ethereum websocket transport".to_string(),
		)
		.into());
	}

	// --- Load cached start
	let last_redeemed = tools::get_cache(
		data_dir.clone(),
		tools::LAST_REDEEMED_CACHE_FILE_NAME,
		Error::NoEthereumStart,
	)
	.await?;
	info!("🌱 Scan ethereum block from: {}", last_redeemed + 1);

	let last_tracked_darwinia_block = tools::get_cache(
		data_dir.clone(),
		tools::LAST_TRACKED_DARWINIA_BLOCK_FILE_NAME,
		Error::NoDarwiniaStart,
	)
	.await? as u32;
	info!(
		"🌱 Scan darwinia block from: {}",
		last_tracked_darwinia_block + 1
	);

	// --- Init APIs ---
	let shadow = Arc::new(Shadow::new(&config));
	let darwinia = Arc::new(Darwinia::new(&config).await?);
	let web3 = Web3::new(Http::new(&config.ethereum.rpc).unwrap());

	// Stop if darwinia sender is authority but without a signer seed
	if darwinia
		.sender
		.is_authority(Some(last_tracked_darwinia_block + 1))
		.await? && darwinia.sender.ethereum_seed.is_none()
	{
		return Err(Error::NoAuthoritySignerSeed.into());
	}

	// --- Network ---
	let runtime_version: sp_version::RuntimeVersion =
		darwinia.client.rpc.runtime_version(None).await?;
	let spec_name = runtime_version.spec_name.to_string();

	// --- Print startup info ---
	info!("🔗 Connect to");
	info!("   Darwinia {}: {}", &spec_name, config.darwinia.rpc);
	info!("   Shadow: {}", config.shadow.endpoint);
	info!("   Ethereum: {}", config.ethereum.rpc);
	let account_id = &darwinia.sender.account_id;
	let roles = darwinia
		.sender
		.role_names(Some(last_tracked_darwinia_block + 1))
		.await?;
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
	start_services(
		&config,
		&shadow,
		&darwinia,
		&web3,
		(
			data_dir,
			spec_name,
			last_redeemed,
			last_tracked_darwinia_block,
		),
	)
	.await
}

async fn start_services(
	config: &Settings,
	shadow: &Arc<Shadow>,
	darwinia: &Arc<Darwinia>,
	web3: &Web3<Http>,
	(data_dir, spec_name, last_redeemed, last_tracked_darwinia_block): (PathBuf, String, u64, u32),
) -> Result<()> {
	// extrinsic sender
	let extrinsics_service =
		ExtrinsicsService::new(darwinia.clone(), spec_name.clone(), data_dir.clone()).start();

	// relay service
	let last_confirmed = darwinia.last_confirmed().await.unwrap();
	let relay_service = RelayService::new(
		shadow.clone(),
		darwinia.clone(),
		last_confirmed,
		config.services.relay.step,
		extrinsics_service.clone().recipient(),
	)
	.start();

	// redeem service
	let redeem_service = RedeemService::new(
		shadow.clone(),
		darwinia.clone(),
		config.services.redeem.step,
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
	let is_tech_comm_member = darwinia
		.sender
		.is_tech_comm_member(Some(last_tracked_darwinia_block + 1))
		.await?;
	let guard_service = GuardService::new(
		shadow.clone(),
		darwinia.clone(),
		config.services.guard.step,
		is_tech_comm_member,
		extrinsics_service.clone().recipient(),
	)
	.map(|g| g.start());

	// darwinia subscribe service
	let ethereum = Ethereum::new(web3.clone(), &config.clone())?;
	let mut subscribe = SubscribeService::new(
		darwinia.clone(),
		ethereum,
		extrinsics_service.clone().recipient(),
		spec_name,
		last_tracked_darwinia_block + 1,
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
