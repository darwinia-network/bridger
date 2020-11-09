//! Redeem Service
use crate::{
    api::{Darwinia, Shadow},
    memcache::EthereumTransactionHash,
    result::Result as BridgerResult,
    result::Error,
};
use primitives::{chain::ethereum::RedeemFor};
use std::{
    sync::Arc,
    time::Duration,
};

use actix::prelude::*;
use crate::memcache::EthereumTransaction;

/// message 'EthereumTransaction'
#[derive(Clone, Debug)]
pub struct MsgEthereumTransaction {
    /// Transaction hash for the event
    pub tx: EthereumTransaction,
}

impl Message for MsgEthereumTransaction {
    type Result = ();
}

/// Redeem Service
pub struct RedeemService {
    step: u64,
    /// Shadow API
    pub shadow: Arc<Shadow>,
    /// Dawrinia API
    pub darwinia: Arc<Darwinia>,
}


impl Actor for RedeemService {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("  🌟 SERVICE STARTED: REDEEM");
    }
}

impl Handler<MsgEthereumTransaction> for RedeemService {
    type Result = AtomicResponse<Self, ()>;

    fn handle(&mut self, msg: MsgEthereumTransaction, _: &mut Context<Self>) -> Self::Result {
        let msg_clone = msg.clone();
        AtomicResponse::new(Box::pin(
            async {}
                .into_actor(self)
                .then(move |_, this, _| {
                    let f = RedeemService::redeem(this.shadow.clone(), this.darwinia.clone(), msg_clone.tx);
                    f.into_actor(this)
                })
                .then(|r, this, ctx| {
                    if let Err(err) = r {
                        if err.to_string().contains("wait") {
                            warn!("{}", err.to_string());
                            ctx.notify_later(msg, Duration::from_millis(this.step * 1000));
                        } else {
                            error!("{}", err.to_string());
                        }
                    }
                    async {Result::<(), Error>::Ok(())}.into_actor(this)
                })
                .map(|_, _, _| {}),
        ))
    }
}

impl RedeemService {
    /// New redeem service
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, step: u64) -> RedeemService {
        RedeemService {
            darwinia,
            shadow,
            step,
        }
    }

    async fn redeem(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, tx: EthereumTransaction) -> BridgerResult<()> {
        trace!("Try to redeem ethereum tx {:?}...", tx.tx_hash);

        // 1. Checking before redeem
        if darwinia.verified(&tx).await? {
            let msg = format!("This ethereum tx {:?} has already been redeemed.", tx.tx_hash);
            return Err(Error::Bridger(msg));
        }

        let last_confirmed = darwinia.last_confirmed().await?;
        if tx.block >= last_confirmed {
            let msg = format!("This ethereum tx {:?}'s block {} not exist, please wait.", tx.tx_hash, tx.block);
            return Err(Error::Bridger(msg));
        }

        // 2. Do redeem
        info!("Prepare to redeem ethereum tx {:?}", tx.tx_hash);
        let proof = shadow
            .receipt(&format!("{:?}", tx.enclosed_hash()), last_confirmed)
            .await?;
        let redeem_for = match tx.tx_hash {
            EthereumTransactionHash::Deposit(_) => RedeemFor::Deposit,
            EthereumTransactionHash::Token(_) => RedeemFor::Token,
        };
        let hash = darwinia.redeem(redeem_for, proof).await?;
        info!("Redeemed with extrinsic {:?}", hash);

        Ok(())
    }

}
