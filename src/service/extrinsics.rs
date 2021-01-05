//! Extrinsics Service
#![allow(missing_docs)]
use std::{sync::Arc, time::Duration};

use actix::prelude::*;

use crate::error::BizError;
use crate::service::redeem::EthereumTransaction;
use crate::service::MsgStop;
use crate::tools;
use crate::{api::Darwinia, error::Result};
use primitives::chain::ethereum::{
	EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor,
};
use primitives::runtime::EcdsaMessage;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Extrinsic {
	Affirm(EthereumRelayHeaderParcel),
	Redeem(RedeemFor, EthereumReceiptProofThing, EthereumTransaction),
	GuardVote(u64, bool),
	SignAndSendMmrRoot(u32),
	SignAndSendAuthorities(EcdsaMessage),
}

/// MsgSign
#[derive(Clone, Debug)]
pub struct MsgExtrinsic(pub Extrinsic);

impl Message for MsgExtrinsic {
	type Result = ();
}

/// Extrinsics Service
pub struct ExtrinsicsService {
	/// Dawrinia API
	pub darwinia: Arc<Darwinia>,

	spec_name: String,
	data_dir: PathBuf,
}

impl Actor for ExtrinsicsService {
	type Context = Context<Self>;

	fn started(&mut self, _: &mut Self::Context) {
		info!("✨ SERVICE STARTED: EX SENDER QUEUE");
	}

	fn stopped(&mut self, _: &mut Self::Context) {
		info!("💤 SERVICE STOPPED: EX SENDER QUEUE")
	}
}

impl Handler<MsgExtrinsic> for ExtrinsicsService {
	type Result = AtomicResponse<Self, ()>;

	fn handle(&mut self, msg: MsgExtrinsic, _: &mut Context<Self>) -> Self::Result {
		AtomicResponse::new(Box::pin(
			async {}
				.into_actor(self)
				.then(|_, this, _| {
					let f = ExtrinsicsService::send_extrinsic(
						this.darwinia.clone(),
						msg.0,
						this.spec_name.clone(),
						this.data_dir.clone(),
					);
					f.into_actor(this)
				})
				.map(|r, _, _| {
					if let Err(err) = r {
						if err.downcast_ref::<BizError>().is_some() {
							trace!("{}", err);
						} else {
							error!("{:?}", err);
						}
					}
				}),
		))
	}
}

impl Handler<MsgStop> for ExtrinsicsService {
	type Result = ();

	fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
		ctx.stop();
	}
}

impl ExtrinsicsService {
	/// New sign service
	pub fn new(darwinia: Arc<Darwinia>, spec_name: String, data_dir: PathBuf) -> ExtrinsicsService {
		ExtrinsicsService {
			darwinia,
			spec_name,
			data_dir,
		}
	}

	async fn send_extrinsic(
		darwinia: Arc<Darwinia>,
		extrinsic: Extrinsic,
		spec_name: String,
		data_dir: PathBuf,
	) -> Result<()> {
		match extrinsic {
			Extrinsic::Affirm(parcel) => {
				let block_number = parcel.header.number;
				let ex_hash = darwinia.affirm(parcel).await?;
				info!(
					"Affirmed ethereum block {} in extrinsic {:?}",
					block_number, ex_hash
				);
			}

			Extrinsic::Redeem(redeem_for, proof, ethereum_tx) => {
				match redeem_for {
					RedeemFor::SetAuthorities => {
						let ex_hash = darwinia.sync_authorities_set(proof, &ethereum_tx.tx_hash).await?;
						info!(
							"Sent ethereum tx {:?} with extrinsic {:?}",
							ethereum_tx.tx_hash, ex_hash
						);
					}
					_ => {
						let ex_hash = darwinia.redeem(redeem_for, proof).await?;
						info!(
							"Redeemed ethereum tx {:?} with extrinsic {:?}",
							ethereum_tx.tx_hash, ex_hash
						);
					}
				}

				// Update cache
				tools::set_cache(
					data_dir,
					tools::LAST_REDEEMED_CACHE_FILE_NAME,
					ethereum_tx.block,
				)
				.await?;
			}

			Extrinsic::GuardVote(pending_block_number, aye) => {
				let ex_hash = darwinia
					.vote_pending_relay_header_parcel(pending_block_number, aye)
					.await?;
				if aye {
					info!(
						"Voted to approve: {}, ex hash: {:?}",
						pending_block_number, ex_hash
					);
				} else {
					info!(
						"Voted to reject: {}, ex hash: {:?}",
						pending_block_number, ex_hash
					);
				}
			}

			Extrinsic::SignAndSendMmrRoot(block_number) => {
				trace!("Start sign and send mmr_root...");
				let ex_hash = darwinia
					.ecdsa_sign_and_submit_signed_mmr_root(spec_name, block_number)
					.await?;
				info!(
					"Sign and send mmr root of block {} in extrinsic {:?}",
					block_number, ex_hash
				);
			}

			Extrinsic::SignAndSendAuthorities(message) => {
				trace!("Start sign and send authorities...");
				let ex_hash = darwinia
					.ecdsa_sign_and_submit_signed_authorities(message)
					.await?;
				info!("Sign and send authorities in extrinsic {:?}", ex_hash);
			}
		}

		// Delay for waiting to fininsh
		tokio::time::delay_for(Duration::from_secs(12)).await;

		Ok(())
	}
}
