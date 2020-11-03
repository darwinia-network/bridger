//! Relay Service
use crate::{api::{Darwinia, Shadow}, result::Error, result::Result as BridgerResult};
use std::sync::Arc;
use substrate_subxt::sp_core::H256;
use primitives::chain::ethereum::EthereumHeader;

use actix::prelude::*;
use actix::fut::Either;

/// message 'block_number'
#[derive(Clone, Debug)]
pub struct MsgBlockNumber(pub u64);

impl Message for MsgBlockNumber {
    type Result = ();
}

/// message 'execute'
#[derive(Clone, Debug)]
pub struct MsgExecute;

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
}

impl Actor for RelayService {
    type Context = Context<Self>;
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
                        let error_reason = "No need to relay".to_string();
                        let f = async {Err(Error::Bridger(error_reason)) };
                        Either::Right(f.into_actor(this))
                    }
                })
                .map(|_, this, _| {
                    this.relayed = this.target
                }),
        ))
    }
}

impl RelayService {

    /// create new relay service actor
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, last_confirmed: u64) -> Self {
        RelayService {
            darwinia,
            shadow,
            target: last_confirmed,
            relayed: last_confirmed,
        }
    }

    /// affirm target block
    pub async fn affirm(darwinia: Arc<Darwinia>, shadow: Arc<Shadow>, target: u64) -> BridgerResult<H256> {
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. last confirmed check
        let last_confirmed = darwinia.last_confirmed().await?;
        if target <= last_confirmed {
            let reason = format!(
                "The target block {} is less than the last_confirmed {}",
                &target, &last_confirmed
            );
            return Err(Error::Bridger(reason));
        }

        // 2. pendings check
        let pending_headers = darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                let reason = format!("The target block {} is pending", &target);
                return Err(Error::Bridger(reason));
            }
        }

        // 3. affirmations check
        for (_game_id, game) in darwinia.affirmations().await?.iter() {
            for (_round_id, affirmations) in game.iter() {
                if Darwinia::contains(&affirmations, target) {
                    let reason = format!("The target block {} is in the relayer game", &target);
                    return Err(Error::Bridger(reason));
                }
            }
        }

        trace!("Prepare to affirm ethereum block: {}", target);
        let parcel = shadow.parcel(target as usize).await?;
        if parcel.header == EthereumHeader::default()
            || parcel.mmr_root == [0u8;32]
        {
            let reason = format!("Shadow service failed to provide parcel for block {}", &target);
            return Err(Error::Bridger(reason));
        }

        // /////////////////////////
        // do affirm
        // /////////////////////////
        match darwinia.affirm(parcel).await {
            Ok(hash) => {
                println!("Affirmed ethereum block {} in extrinsic {:?}", target, hash);
                Ok(hash)
            }
            Err(err) => Err(err),
        }
    }
}
