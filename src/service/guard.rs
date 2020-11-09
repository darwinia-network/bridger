//! Guard Service
use std::{
    sync::Arc, time::Duration
};
use actix::prelude::*;

use crate::{
    api::{Darwinia, Shadow},
    result::{Result as BridgerResult},
};
use crate::result::Error::Bridger;

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
}

impl Actor for GuardService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("   🌟 SERVICE STARTED: GUARD");
        ctx.run_interval(Duration::from_millis(self.step * 1_000),  |_this, ctx| {
            ctx.notify(MsgGuard {});
        });
    }
}

impl Handler<MsgGuard> for GuardService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, _msg: MsgGuard, _: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(|_, this, _| {
                    let f = GuardService::guard(this.shadow.clone(), this.darwinia.clone());
                    f.into_actor(this)
                })
                .map(|_, _, _| {}),
        ))
    }
}

impl GuardService {
    /// New redeem service
    pub async fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, step: u64) -> BridgerResult<GuardService> {
        let is_tech_comm_member = darwinia.account.is_tech_comm_member().await?;

        if is_tech_comm_member {
            Ok(GuardService {
                darwinia,
                shadow,
                step,
            })
        } else {
            info!("   💩 GUARD SERVICE NOT STARTED, YOU ARE NOT TECH COMM MEMBER");
            Err(Bridger("Not tech comm member".to_string()))
        }
    }

    async fn guard(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>) -> BridgerResult<()> {
        trace!("Checking pending headers...");

        let pending_headers = darwinia.pending_headers().await?;
        for pending in pending_headers {
            let pending_parcel = pending.1;
            let voting_state = pending.2;

            if !darwinia.account.has_voted(voting_state) {
                let pending_block_number: u64 = pending_parcel.header.number;
                let parcel_from_shadow = shadow.parcel(pending_block_number as usize).await?;

                let parcel_fulfilled = !(
                    parcel_from_shadow.header.hash.is_none()
                    || parcel_from_shadow.header.hash.unwrap() == [0u8; 32]
                    || parcel_from_shadow.mmr_root == [0u8; 32]
                );

                if parcel_fulfilled {
                    // delay to wait for possible previous extrinsics
                    tokio::time::delay_for(Duration::from_secs(12)).await;
                    if pending_parcel.is_same_as(&parcel_from_shadow) {
                        darwinia.vote_pending_relay_header_parcel(pending_block_number, true).await?;
                        info!("Voted to approve {}", pending_block_number);
                    } else {
                        darwinia.vote_pending_relay_header_parcel(pending_block_number, false).await?;
                        info!("Voted to reject {}", pending_block_number);
                    };
                }
            }
        }

        Ok(())
    }
}
