//! Redeem Service
use crate::{
    api::{Darwinia, Shadow},
    error::Result,
    error::Error,
};
use primitives::{chain::ethereum::RedeemFor};
use std::{
    sync::Arc,
    time::Duration,
};
use actix::prelude::*;

use std::cmp::{Ord, Ordering, PartialOrd};
use web3::types::H256;
use crate::service::MsgStop;
use crate::error::Error::Bridger;
use tokio::fs::File;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

    data_dir: PathBuf,
}


impl Actor for RedeemService {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("   ✨ SERVICE STARTED: REDEEM");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("   💤 SERVICE STOPPED: REDEEM")
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
                    let f = RedeemService::redeem(this.shadow.clone(), this.darwinia.clone(), msg_clone.tx, this.data_dir.clone());
                    f.into_actor(this)
                })
                .then(|r, this, ctx| {
                    if let Err(err) = r {
                        if let Error::RedeemingBlockLargeThanLastConfirmed(..) = err {
                            warn!("{}, please wait!", err);
                            ctx.notify_later(msg, Duration::from_millis(this.step * 1000));
                        } else {
                            warn!("{}", err);
                        }
                    }
                    async {Result::<()>::Ok(())}.into_actor(this)
                })
                .map(|_, _, _| {}),
        ))
    }
}

impl Handler<MsgStop> for RedeemService {
    type Result = ();

    fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl RedeemService {
    /// New redeem service
    pub fn new(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, step: u64, data_dir: PathBuf) -> RedeemService {
        RedeemService {
            darwinia,
            shadow,
            step,
            data_dir,
        }
    }

    async fn redeem(shadow: Arc<Shadow>, darwinia: Arc<Darwinia>, tx: EthereumTransaction, data_dir: PathBuf) -> Result<()> {
        info!("Try to redeem ethereum tx {:?}...", tx.tx_hash);

        // 1. Checking before redeem
        if darwinia.verified(&tx).await? {
            return Err(Error::TxRedeemed(tx.tx_hash));
        }

        let last_confirmed = darwinia.last_confirmed().await?;
        if tx.block >= last_confirmed {
            return Err(Error::RedeemingBlockLargeThanLastConfirmed(tx.tx_hash, tx.block, last_confirmed));
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
        info!("Redeemed ethereum tx {:?} with extrinsic {:?}", tx.enclosed_hash(), hash);

        // 3. Update cache
        RedeemService::set_last_redeemed(data_dir, tx.block).await?;
        Ok(())
    }

    const LAST_REDEEMED_CACHE_FILE_NAME: &'static str = "last-redeemed";

    /// Get last redeemed block number
    pub async fn get_last_redeemed(data_dir: PathBuf) -> Result<u64> {
        let mut filepath = data_dir;
        filepath.push(RedeemService::LAST_REDEEMED_CACHE_FILE_NAME);

        // if cache file not exist
        if File::open(&filepath).await.is_err() {
            return Err(Error::LastRedeemedFileNotExists);
        }

        // read start from cache file
        let mut file = File::open(filepath).await?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        match buffer.trim().parse() {
            Ok(start) => Ok(start),
            Err(e) => Err(Bridger(e.to_string()))
        }
    }

    /// Set last redeemed block number
    pub async fn set_last_redeemed(data_dir: PathBuf, value: u64) -> Result<()> {
        let mut filepath = data_dir;
        filepath.push(RedeemService::LAST_REDEEMED_CACHE_FILE_NAME);
        let mut file = File::create(filepath).await?;
        file.write_all(value.to_string().as_bytes()).await?;
        Ok(())
    }

}
