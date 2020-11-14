//! Guard Service
use std::{
    sync::Arc, time::Duration
};
use actix::prelude::*;

use crate::{
    api::{Darwinia, Shadow},
    result::{Result as BridgerResult},
};
use crate::service::MsgStop;

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

    voted: Vec<u64>,
}

impl Actor for GuardService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("    ✨ SERVICE STARTED: GUARD");
        ctx.run_interval(Duration::from_millis(self.step * 1_000),  |_this, ctx| {
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
                    let f = GuardService::guard(this.shadow.clone(), this.darwinia.clone(), this.voted.clone());
                    f.into_actor(this)
                })
                .map(|r, this, _| {
                    match r {
                        Ok(mut vote_result) => {
                            this.voted.append(&mut vote_result);
                        },
                        Err(e) => {
                            warn!("{}", e.to_string());
                        },
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
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, step: u64, is_tech_comm_member: bool) -> Option<GuardService> {
        if is_tech_comm_member {
            Some(GuardService {
                darwinia,
                shadow,
                step,
                voted: vec![]
            })
        } else {
            warn!("    🙌 GUARD SERVICE NOT STARTED, YOU ARE NOT TECH COMM MEMBER");
            None
        }
    }

    async fn guard(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, voted: Vec<u64>) -> BridgerResult<Vec<u64>> {
        trace!("Checking pending headers...");
        let mut result = vec![];

        let last_confirmed = darwinia.last_confirmed().await.unwrap();
        let pending_headers = darwinia.pending_headers().await?;
        for pending in pending_headers {
            let pending_parcel = pending.1;
            let voting_state = pending.2;
            let pending_block_number: u64 = pending_parcel.header.number;

            // https://github.com/darwinia-network/bridger/issues/33
            if pending_block_number <= last_confirmed {
                continue;
            }

            // Local voted check
            if voted.contains(&pending_block_number) {
                continue;
            }

            // On chain voted check
            if darwinia.account.has_voted(voting_state) {
                continue;
            }

            let parcel_from_shadow = shadow.parcel(pending_block_number as usize).await?;

            // Parcel from shadow fulfilled check
            let parcel_fulfilled = !(
                parcel_from_shadow.header.hash.is_none()
                || parcel_from_shadow.header.hash.unwrap() == [0u8; 32]
                || parcel_from_shadow.mmr_root == [0u8; 32]
            );
            if !parcel_fulfilled {
                continue;
            }

            // Delay to wait for possible previous extrinsics
            tokio::time::delay_for(Duration::from_secs(12)).await;

            // Do vote
            if pending_parcel.is_same_as(&parcel_from_shadow) {
                let ex_hash = darwinia.vote_pending_relay_header_parcel(pending_block_number, true).await?;
                info!("Voted to approve: {}, ex hash: {:?}", pending_block_number, ex_hash);
            } else {
                let ex_hash = darwinia.vote_pending_relay_header_parcel(pending_block_number, false).await?;
                info!("Voted to reject: {}, ex hash: {:?}", pending_block_number, ex_hash);
            };

            result.push(pending_block_number);
        }

        Ok(result)
    }
}
