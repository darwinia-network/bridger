//! Relay Service
use crate::{api::{Darwinia, Shadow}, error::Error, error::Result};
use std::sync::Arc;
use primitives::chain::ethereum::EthereumHeader;

use actix::prelude::*;
use actix::fut::Either;
use std::time::Duration;
use crate::service::MsgStop;

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
    pub darwinia: Arc<Darwinia>,

    target: u64,
    relayed: u64,
    step: u64,
}

impl Actor for RelayService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("    ✨ SERVICE STARTED: RELAY");
        ctx.run_interval(Duration::from_millis(self.step * 1_000),  |_this, ctx| {
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
        if msg.0> self.relayed && msg.0 > self.target {
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
                        let f = RelayService::affirm(this.darwinia.clone(), this.shadow.clone(), this.target);
                        Either::Left(f.into_actor(this))
                    } else {
                        let f = async {Ok(())};
                        Either::Right(f.into_actor(this))
                    }
                })
                .map(|r, this, _| {
                    match r {
                        Ok(_) => this.relayed = this.target,
                        Err(e@Error::AffirmingBlockLessThanLastConfirmed(..)) => {
                            this.relayed = this.target;
                            warn!("{}", e);
                        },
                        Err(e) => {
                            warn!("{}", e);
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
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, last_confirmed: u64, step: u64) -> Self {
        RelayService {
            darwinia,
            shadow,
            target: last_confirmed,
            relayed: last_confirmed,
            step,
        }
    }

    /// affirm target block
    pub async fn affirm(darwinia: Arc<Darwinia>, shadow: Arc<Shadow>, target: u64) -> Result<()> {
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. last confirmed check
        let last_confirmed = darwinia.last_confirmed().await?;
        if target <= last_confirmed {
            return Err(Error::AffirmingBlockLessThanLastConfirmed(target, last_confirmed));
        }

        // 2. pendings check
        let pending_headers = darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                return Err(Error::AffirmingBlockInPending(target));
            }
        }

        // 3. affirmations check
        for (_game_id, game) in darwinia.affirmations().await?.iter() {
            for (_round_id, affirmations) in game.iter() {
                if Darwinia::contains(&affirmations, target) {
                    return Err(Error::AffirmingBlockInGame(target));
                }
            }
        }

        trace!("Prepare to affirm ethereum block: {}", target);
        let parcel = shadow.parcel(target as usize).await?;
        if parcel.header == EthereumHeader::default()
            || parcel.mmr_root == [0u8;32]
        {
            return Err(Error::AffirmingBlockInGame(target));
        }

        // /////////////////////////
        // do affirm
        // /////////////////////////
        match darwinia.affirm(parcel).await {
            Ok(hash) => {
                info!("Affirmed ethereum block {} in extrinsic {:?}", target, hash);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
