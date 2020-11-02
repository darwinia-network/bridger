//! Relay Service
use crate::{api::{Darwinia, Shadow}, result::Error, result::Result as BridgerResult};
use std::sync::Arc;
use substrate_subxt::sp_core::H256;
use primitives::chain::ethereum::EthereumHeader;

use actix::prelude::*;
use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
pub struct MsgBlockNumber(u64);

impl Message for MsgBlockNumber {
    type Result = ();
}

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
}

impl Actor for RelayService {
    type Context = Context<Self>;
}

impl Handler<MsgBlockNumber> for RelayService {
    type Result = ();

    fn handle(&mut self, msg: MsgBlockNumber, ctx: &mut Context<Self>) -> Self::Result {
        if msg.0 > self.target {
            self.target = msg.0;
        }
    }
}

impl Handler<MsgExecute> for RelayService {
    type Result = ();

    fn handle(&mut self, msg: MsgExecute, ctx: &mut Context<Self>) -> Self::Result {
        futures::executor::block_on(async {
            println!("hello");
            self.execute().await;
        });

    }
}


impl RelayService {

    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, start_block: u64) -> Self {
        RelayService {
            darwinia,
            shadow,
            target: start_block,
        }
    }

    async fn execute(&self) {
        println!("---------------");
        if let Ok(ex_hash) = self.affirm(self.target).await {
            info!("{:?}", ex_hash);
        }
    }

    /// affirm target block
    pub async fn affirm(&self, target: u64) -> BridgerResult<H256> {
        println!("---3------------");
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. last confirmed check
        let last_confirmed = self.darwinia.last_confirmed().await?;
        println!("---a------------");
        if target <= last_confirmed {
            let reason = format!(
                "The target block {} is less than the last_confirmed {}",
                &target, &last_confirmed
            );
            return Err(Error::Bridger(reason));
        }
        println!("---4------------");

        // 2. pendings check
        let pending_headers = self.darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                let reason = format!("The target block {} is pending", &target);
                return Err(Error::Bridger(reason));
            }
        }

        println!("---5------------");
        // 3. affirmations check
        for (_game_id, game) in self.darwinia.affirmations().await?.iter() {
            for (_round_id, affirmations) in game.iter() {
                if Darwinia::contains(&affirmations, target) {
                    let reason = format!("The target block {} is in the relayer game", &target);
                    return Err(Error::Bridger(reason));
                }
            }
        }
        println!("---6------------");

        trace!("Prepare to affirm ethereum block: {}", target);
        let parcel = self.shadow.parcel(target as usize).await?;
        println!("---6------------{:?}", parcel);
        // if parcel.header == EthereumHeader::default()
        //     || parcel.mmr_root == [0u8;32]
        // {
        //     let reason = format!("Shadow service failed to provide parcel for block {}", &target);
        //     return Err(Error::Bridger(reason));
        // }

        println!("---7------------");
        // /////////////////////////
        // do affirm
        // /////////////////////////
        match self.darwinia.affirm(parcel).await {
            Ok(hash) => {
                println!("Affirmed ethereum block {} in extrinsic {:?}", target, hash);
                Ok(hash)
            }
            Err(err) => Err(err),
        }
    }
}
