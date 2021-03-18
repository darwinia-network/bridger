//! Relay Service
use crate::{api::Shadow, error::Result};
use primitives::chain::ethereum::EthereumHeader;
use std::sync::Arc;

use crate::error::BizError;
use crate::service::extrinsics::{Extrinsic, MsgExtrinsic};
use crate::service::MsgStop;
use actix::fut::Either;
use actix::prelude::*;
use anyhow::Context as AnyhowContext;
use std::time::Duration;

use crate::tools;
use darwinia::Ethereum2Darwinia;

/// message 'block_number'
#[derive(Clone, Debug)]
pub struct MsgBlockNumber(pub u64);

impl Message for MsgBlockNumber {
	type Result = ();
}

/// message 'execute'
#[derive(Clone, Debug)]
struct MsgExecute;

impl Message for MsgExecute {
	type Result = ();
}

/// Relay Service
pub struct RelayService {
	/// Shadow API
	pub shadow: Arc<Shadow>,
	/// Dawrinia API
	pub ethereum2darwinia: Ethereum2Darwinia,

	target: u64,
	relayed: u64,
	step: u64,

	extrinsics_service: Recipient<MsgExtrinsic>,
}

impl Actor for RelayService {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		info!("    ✨ SERVICE STARTED: RELAY");
		ctx.run_interval(Duration::from_millis(self.step * 1_000), |_this, ctx| {
			ctx.notify(MsgExecute {});
		});
	}

	fn stopped(&mut self, _: &mut Self::Context) {
		info!("    💤 SERVICE STOPPED: RELAY")
	}
}

impl Handler<MsgBlockNumber> for RelayService {
	type Result = ();

	fn handle(&mut self, msg: MsgBlockNumber, _: &mut Context<Self>) -> Self::Result {
		if msg.0 > self.relayed && msg.0 > self.target {
			self.target = msg.0;
		}
	}
}

impl Handler<MsgExecute> for RelayService {
	type Result = AtomicResponse<Self, ()>;

	fn handle(&mut self, _: MsgExecute, _: &mut Context<Self>) -> Self::Result {
		AtomicResponse::new(Box::pin(
			async {}
				.into_actor(self)
				.then(|_, this, _| {
					if this.target > this.relayed {
						let f = RelayService::affirm(
							this.ethereum2darwinia.clone(),
							this.shadow.clone(),
							this.target,
							this.extrinsics_service.clone(),
						);
						Either::Left(f.into_actor(this))
					} else {
						let f = async { Ok(()) };
						Either::Right(f.into_actor(this))
					}
				})
				.map(|r, this, _| {
					match r {
						Ok(_) => this.relayed = this.target,
						Err(err) => {
							if let Some(e) = err.downcast_ref::<BizError>() {
								match e {
									BizError::AffirmingBlockLessThanLastConfirmed(..) => {
										this.relayed = this.target; // not try again
										trace!("{}", err);
									}
									_ => trace!("{}", err),
								}
							} else {
								error!("{:#?}", err);
							}
						}
					}
				}),
		))
	}
}

impl Handler<MsgStop> for RelayService {
	type Result = ();

	fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
		ctx.stop();
	}
}

impl RelayService {
	/// create new relay service actor
	pub fn new(
		shadow: Arc<Shadow>,
		ethereum2darwinia: Ethereum2Darwinia,
		last_confirmed: u64,
		step: u64,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Self {
		RelayService {
			ethereum2darwinia,
			shadow,
			target: last_confirmed,
			relayed: last_confirmed,
			step,
			extrinsics_service,
		}
	}

	/// affirm target block
	pub async fn affirm(
		ethereum2darwinia: Ethereum2Darwinia,
		shadow: Arc<Shadow>,
		target: u64,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Result<()> {
		// /////////////////////////
		// checking before affirm
		// /////////////////////////
		// 1. last confirmed check
		let last_confirmed = ethereum2darwinia.last_confirmed().await?;
		if target <= last_confirmed {
			return Err(
				BizError::AffirmingBlockLessThanLastConfirmed(target, last_confirmed).into(),
			);
		}

		// 2. pendings check
		let pending_headers = ethereum2darwinia.pending_headers().await?;
		for pending_header in pending_headers {
			let pending_block_number = pending_header.1.header.number;
			if pending_block_number >= target {
				return Err(BizError::AffirmingBlockInPending(target).into());
			}
		}

		// 3. affirmations check
		for (_game_id, game) in ethereum2darwinia.affirmations().await?.iter() {
			for (_round_id, affirmations) in game.iter() {
				if Ethereum2Darwinia::contains(&affirmations, target) {
					return Err(BizError::AffirmingBlockInGame(target).into());
				}
			}
		}

		trace!("Prepare to affirm ethereum block: {}", target);
		let parcel = shadow.parcel(target as usize + 1).await.with_context(|| {
			format!(
				"Fail to get parcel from shadow when affirming ethereum block {}",
				target
			)
		})?;
		if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
			return Err(BizError::ParcelFromShadowIsEmpty(target).into());
		}

		// /////////////////////////
		// do affirm
		// /////////////////////////
		let ex = Extrinsic::Affirm(parcel);
		tools::send_extrinsic(&extrinsics_service, ex).await;

		Ok(())
	}
}
