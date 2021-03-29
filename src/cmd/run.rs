use crate::api::{darwinia_api, Shadow};
use crate::{
	// listener::Listener,
	error::{Error, Result},
	Settings,
};
use actix::Actor;
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

use darwinia::{
	Darwinia, Darwinia2Ethereum, Ethereum2Darwinia, FromEthereumAccount, ToEthereumAccount,
};

/// Run the bridger
pub async fn exec(data_dir: Option<PathBuf>, verbose: bool) -> Result<()> {
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
	let mut config = Settings::new(&data_dir)?;

	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:").unwrap();
		config.decrypt(&passwd)?;
	}
	loop {
		if let Err(e) = run(data_dir.clone(), &config).await {
			error!("{:?}", e);
			match e.downcast_ref() {
				Some(Error::NoDarwiniaStart) | Some(Error::NoEthereumStart) => {
					// performing retry
					info!("Bridger will restart in 30 seconds...");
					time::delay_for(Duration::from_secs(30)).await;
				}
				// break default
				_ => return Err(e),
			}
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
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia.clone());
	let darwinia2ethereum = darwinia_api::get_d2e_instance(darwinia.clone());
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let from_ethereum_account = darwinia_api::get_e2d_account(darwinia_account.clone());
	let to_ethereum_account = darwinia_api::get_d2e_account(darwinia_account.clone(), &config);

	let web3 = Web3::new(
		Http::new(&config.ethereum.rpc)
			.map_err(|e| Error::NewHttpError(config.ethereum.rpc.clone(), e.to_string()))?,
	);

	// Stop if darwinia sender is authority but without a signer seed
	if darwinia2ethereum
		.is_authority(Some(last_tracked_darwinia_block + 1), &to_ethereum_account)
		.await? && !to_ethereum_account.has_ethereum_seed()
	{
		return Err(Error::NoAuthoritySignerSeed.into());
	}

	// --- Network ---
	let spec_name = darwinia.runtime_version().await?;

	// --- Print startup info ---
	info!("🔗 Connect to");
	info!("   Darwinia {}: {}", &spec_name, config.darwinia.rpc);
	info!("   Shadow: {}", config.shadow.endpoint);
	info!("   Ethereum: {}", config.ethereum.rpc);
	darwinia2ethereum
		.account_detail(Some(last_tracked_darwinia_block + 1), &to_ethereum_account)
		.await?;
	ethereum2darwinia
		.account_detail(
			Some(last_tracked_darwinia_block + 1),
			&from_ethereum_account,
		)
		.await?;

	// --- Start services ---
	start_services(
		&config,
		&shadow,
		&darwinia,
		Some(ethereum2darwinia),
		Some(darwinia2ethereum),
		Some(from_ethereum_account),
		Some(to_ethereum_account),
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

#[allow(clippy::too_many_arguments)]
async fn start_services(
	config: &Settings,
	shadow: &Arc<Shadow>,
	darwinia: &Darwinia,
	ethereum2darwinia: Option<Ethereum2Darwinia>,
	darwinia2ethereum: Option<Darwinia2Ethereum>,
	ethereum2darwinia_relayer: Option<FromEthereumAccount>,
	darwinia2ethereum_relayer: Option<ToEthereumAccount>,
	web3: &Web3<Http>,
	(data_dir, spec_name, last_redeemed, last_tracked_darwinia_block): (PathBuf, String, u64, u32),
) -> Result<()> {
	// extrinsic sender
	let extrinsics_service = ExtrinsicsService::new(
		ethereum2darwinia.clone(),
		darwinia2ethereum.clone(),
		ethereum2darwinia_relayer.clone(),
		darwinia2ethereum_relayer.clone(),
		spec_name.clone(),
		data_dir.clone(),
	)
	.start();

	let (relay_service, redeem_service, ethereum_service, guard_service) = {
		if let Some(ethereum2darwinia) = &ethereum2darwinia {
			// relay service
			let last_confirmed = ethereum2darwinia.last_confirmed().await.unwrap();
			let relay_service = RelayService::new(
				shadow.clone(),
				ethereum2darwinia.clone(),
				last_confirmed,
				config.services.relay.step,
				extrinsics_service.clone().recipient(),
			)
			.start();

			// redeem service
			let redeem_service = RedeemService::new(
				shadow.clone(),
				ethereum2darwinia.clone(),
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
			let guard_service = if let Some(relayer) = &ethereum2darwinia_relayer {
				let is_tech_comm_member = ethereum2darwinia
					.is_tech_comm_member(Some(last_tracked_darwinia_block + 1), &relayer)
					.await?;
				GuardService::new(
					shadow.clone(),
					ethereum2darwinia.clone(),
					relayer.clone(),
					config.services.guard.step,
					is_tech_comm_member,
					extrinsics_service.clone().recipient(),
				)
				.map(|g| g.start())
			} else {
				None
			};
			(
				Some(relay_service),
				Some(redeem_service),
				Some(ethereum_service),
				guard_service,
			)
		} else {
			(None, None, None, None)
		}
	};

	if let Some(darwinia2ethereum) = &darwinia2ethereum {
		// darwinia subscribe service
		let ethereum = Ethereum::new(web3.clone(), &config.clone())?;
		let mut subscribe = SubscribeService::new(
			darwinia2ethereum.clone(),
			darwinia2ethereum_relayer.unwrap(),
			ethereum,
			extrinsics_service.clone().recipient(),
			spec_name,
			last_tracked_darwinia_block + 1,
			data_dir.clone(),
		);

		if let Err(_e) = subscribe.start().await {
			if let Some(ethereum_service) = &ethereum_service {
				ethereum_service.do_send(MsgStop {});
			}
			if let Some(relay_service) = &relay_service {
				relay_service.do_send(MsgStop {});
			}
			if let Some(redeem_service) = &redeem_service {
				redeem_service.do_send(MsgStop {});
			}
			if let Some(guard_service) = &guard_service {
				guard_service.do_send(MsgStop {});
			}
			subscribe.stop();
			extrinsics_service.do_send(MsgStop {});
		}
	}
	Ok(())
}
