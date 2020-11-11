//! Redeem Service
use crate::{
    api::{Darwinia, Shadow},
    result::Result as BridgerResult,
    result::Error,
};
use primitives::{chain::ethereum::RedeemFor};
use std::{
    sync::Arc,
    time::Duration,
};
use actix::prelude::*;

use std::cmp::{Ord, Ordering, PartialOrd};
use web3::types::H256;

/// Ethereum transaction event with hash
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EthereumTransactionHash {
    /// Deposit event
    Deposit(H256),
    /// Token event
    Token(H256),
}

/// Reedeemable Ethereum transaction
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EthereumTransaction {
    /// Transaction hash for the event
    pub tx_hash: EthereumTransactionHash,
    /// Block Hash for the event
    pub block_hash: H256,
    /// Transaction block
    pub block: u64,
    /// Transaction index
    pub index: u64,
}

impl EthereumTransaction {
    /// Get the hash
    pub fn enclosed_hash(&self) -> H256 {
        match self.tx_hash {
            EthereumTransactionHash::Token(h) => h,
            EthereumTransactionHash::Deposit(h) => h,
        }
    }
}

impl PartialOrd for EthereumTransaction {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        self.block.partial_cmp(&o.block)
    }
}

impl Ord for EthereumTransaction {
    fn cmp(&self, o: &Self) -> Ordering {
        self.block.cmp(&o.block)
    }
}

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
        info!("   🌟 SERVICE STARTED: REDEEM");
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
                            warn!("{}", err.to_string());
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
        info!("      Try to redeem ethereum tx {:?}...", tx.tx_hash);

        // 1. Checking before redeem
        if darwinia.verified(&tx).await? {
            let msg = format!("      This ethereum tx {:?} has already been redeemed.", tx.enclosed_hash());
            return Err(Error::Bridger(msg));
        }

        let last_confirmed = darwinia.last_confirmed().await?;
        if tx.block >= last_confirmed {
            let msg = format!("      This ethereum tx {:?}'s block {} not confirmed, please wait.", tx.enclosed_hash(), tx.block);
            return Err(Error::Bridger(msg));
        }

        // 2. Do redeem
        let proof = shadow
            .receipt(&format!("{:?}", tx.enclosed_hash()), last_confirmed)
            .await?;
        let redeem_for = match tx.tx_hash {
            EthereumTransactionHash::Deposit(_) => RedeemFor::Deposit,
            EthereumTransactionHash::Token(_) => RedeemFor::Token,
        };
        let hash = darwinia.redeem(redeem_for, proof).await?;
        info!("      Redeemed ethereum tx {:?} with extrinsic {:?}", tx.enclosed_hash(), hash);

        Ok(())
    }

}
