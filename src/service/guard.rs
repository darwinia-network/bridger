//! Guard Service
use actix::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::service::extrinsics::{Extrinsic, MsgExtrinsic};
use crate::service::MsgStop;
use crate::{
	api::{Darwinia, Shadow},
	error::{BizError, Result},
};

#[derive(Clone, Debug)]
struct MsgGuard;

impl Message for MsgGuard {
	type Result = ();
}

/// Redeem Service
pub struct GuardService {
	step: u64,
	/// Shadow API
	pub shadow: Arc<Shadow>,
	/// Dawrinia API
	pub darwinia: Arc<Darwinia>,
	extrinsics_service: Recipient<MsgExtrinsic>,
}

impl Actor for GuardService {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		info!("    ✨ SERVICE STARTED: GUARD");
		ctx.run_interval(Duration::from_millis(self.step * 1_000), |_this, ctx| {
			ctx.notify(MsgGuard {});
		});
	}

	fn stopped(&mut self, _: &mut Self::Context) {
		info!("    💤 SERVICE STOPPED: GUARD")
	}
}

impl Handler<MsgGuard> for GuardService {
	type Result = AtomicResponse<Self, ()>;

	fn handle(&mut self, _msg: MsgGuard, _: &mut Context<Self>) -> Self::Result {
		AtomicResponse::new(Box::pin(
			async {}
				.into_actor(self)
				.then(|_, this, _| {
					let f = GuardService::guard(
						this.shadow.clone(),
						this.darwinia.clone(),
						this.extrinsics_service.clone(),
					);
					f.into_actor(this)
				})
				.map(|r, _this, _| {
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

impl Handler<MsgStop> for GuardService {
	type Result = ();

	fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
		ctx.stop();
	}
}

impl GuardService {
	/// New redeem service
	pub fn new(
		shadow: Arc<Shadow>,
		darwinia: Arc<Darwinia>,
		step: u64,
		is_tech_comm_member: bool,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Option<GuardService> {
		if is_tech_comm_member {
			Some(GuardService {
				darwinia,
				shadow,
				step,
				extrinsics_service,
			})
		} else {
			warn!("    🔒 GUARD SERVICE NOT STARTED, YOU ARE NOT TECH COMM MEMBER");
			None
		}
	}

	async fn guard(
		shadow: Arc<Shadow>,
		darwinia: Arc<Darwinia>,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Result<()> {
		trace!("Checking pending headers...");

		let last_confirmed = darwinia.last_confirmed().await.unwrap();
		let pending_headers = darwinia.pending_headers().await?;
		if !pending_headers.is_empty() {
			trace!(
				"pending headers: {:?}",
				pending_headers
					.clone()
					.iter()
					.map(|p| p.1.header.number.to_string())
					.collect::<Vec<_>>()
					.join(", ")
			);
		}
		for pending in pending_headers {
			let pending_parcel = pending.1;
			let voting_state = pending.2;
			let pending_block_number: u64 = pending_parcel.header.number;

			// high than last_confirmed(https://github.com/darwinia-network/bridger/issues/33),
			// and,
			// have not voted
			if pending_block_number > last_confirmed && !darwinia.sender.has_voted(voting_state) {
				let parcel_from_shadow = shadow.parcel(pending_block_number as usize).await?;
				let ex = if pending_parcel.is_same_as(&parcel_from_shadow) {
					Extrinsic::GuardVote(pending_block_number, true)
				} else {
					Extrinsic::GuardVote(pending_block_number, false)
				};
				extrinsics_service.send(MsgExtrinsic(ex)).await?;
			}
		}

		Ok(())
	}
}
