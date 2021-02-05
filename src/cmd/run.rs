use crate::api::Shadow;
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

use darwinia::{
	Darwinia, Darwinia2Ethereum, DarwiniaAccount, Ethereum2Darwinia, FromEthereumAccount,
	ToEthereumAccount,
};

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
	let darwinia = Darwinia::new(&config.node).await?;
	let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
	let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
	let darwinia_account = DarwiniaAccount::new(
		config.seed.clone(),
		config
			.proxy
			.clone()
			.map(|proxy| proxy.real[2..].to_string()),
	);
	let from_ethereum_account = FromEthereumAccount::new(darwinia_account.clone());
	let to_ethereum_account = ToEthereumAccount::new(
		darwinia_account.clone(),
		config.darwinia_to_ethereum.seed.clone(),
		config.eth.rpc.to_string(),
	);

	let web3 = Web3::new(Http::new(&config.eth.rpc).unwrap());

	// Stop if darwinia sender is authority but without a signer seed
	if darwinia2ethereum.is_authority(&to_ethereum_account).await?
		&& !to_ethereum_account.has_ethereum_seed()
	{
		return Err(Error::NoAuthoritySignerSeed.into());
	}

	// --- Network ---
	let spec_name = darwinia.runtime_version().await?;

	// --- Print startup info ---
	info!("🔗 Connect to");
	info!("   Darwinia {}: {}", &spec_name, config.node);
	info!("   Shadow: {}", config.shadow);
	info!("   Ethereum: {}", config.eth.rpc);
	darwinia2ethereum
		.account_detail(&to_ethereum_account)
		.await?;
	ethereum2darwinia
		.account_detail(&from_ethereum_account)
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
		data_dir,
		spec_name,
	)
	.await
}

#[allow(clippy::too_many_arguments)]
async fn start_services(
	config: &Config,
	shadow: &Arc<Shadow>,
	darwinia: &Darwinia,
	ethereum2darwinia: Option<Ethereum2Darwinia>,
	darwinia2ethereum: Option<Darwinia2Ethereum>,
	ethereum2darwinia_relayer: Option<FromEthereumAccount>,
	darwinia2ethereum_relayer: Option<ToEthereumAccount>,
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
				config.step.relay,
				extrinsics_service.clone().recipient(),
			)
			.start();

			// redeem service
			let redeem_service = RedeemService::new(
				shadow.clone(),
				ethereum2darwinia.clone(),
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
			let guard_service = if let Some(relayer) = &ethereum2darwinia_relayer {
				let is_tech_comm_member = ethereum2darwinia.is_tech_comm_member(&relayer).await?;
				GuardService::new(
					shadow.clone(),
					ethereum2darwinia.clone(),
					relayer.clone(),
					config.step.guard,
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
			(last_tracked_ethereum_block as u32) + 1,
			data_dir.clone(),
		);
		let b = async {
			if let Err(e) = subscribe.start().await {
				return Err(e);
			}
			Ok(())
		};

		let killer = darwinia.subxt.rpc.client.killer.clone();
		let c = async {
			loop {
				if killer.lock().await.next().await.is_some() {
					return Err(BizError::Bridger("Jsonrpsee's ws connection closed".into()).into());
				}
			}
		};

		if let Err(e) = select!(b, c).await {
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
			return Err(e);
		}
	}
	Ok(())
}
