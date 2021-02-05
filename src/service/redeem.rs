//! Redeem Service
use crate::{api::Shadow, error::Result};
use actix::prelude::*;
use primitives::chain::ethereum::RedeemFor;
use std::{sync::Arc, time::Duration};

use crate::error::BizError;
use crate::service::extrinsics::{Extrinsic, MsgExtrinsic};
use crate::service::MsgStop;
use std::cmp::{Ord, Ordering, PartialOrd};
use web3::types::H256;

use darwinia::Ethereum2Darwinia;

/// Ethereum transaction event with hash
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EthereumTransactionHash {
	/// Deposit event
	Deposit(H256),
	/// Token event
	Token(H256),
	/// SetAuthoritiesEvent
	SetAuthorities(H256),
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
			EthereumTransactionHash::SetAuthorities(h) => h,
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
	pub ethereum2darwinia: Ethereum2Darwinia,

	extrinsics_service: Recipient<MsgExtrinsic>,
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
					let f = RedeemService::redeem(
						this.ethereum2darwinia.clone(),
						this.shadow.clone(),
						msg_clone.tx,
						this.extrinsics_service.clone(),
					);
					f.into_actor(this)
				})
				.map(|r, this, ctx| {
					if let Err(err) = r {
						if let Some(e) = err.downcast_ref::<BizError>() {
							match e {
								BizError::RedeemingBlockLargeThanLastConfirmed(..) => {
									trace!("{}, please wait!", err);
									ctx.notify_later(msg, Duration::from_millis(this.step * 1000));
								}
								_ => trace!("{}", err),
							}
						} else {
							error!("{:?}", err);
						}
					}
				}),
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
	pub fn new(
		shadow: Arc<Shadow>,
		ethereum2darwinia: Ethereum2Darwinia,
		step: u64,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> RedeemService {
		RedeemService {
			ethereum2darwinia,
			shadow,
			step,
			extrinsics_service,
		}
	}

	async fn redeem(
		ethereum2darwinia: Ethereum2Darwinia,
		shadow: Arc<Shadow>,
		tx: EthereumTransaction,
		extrinsics_service: Recipient<MsgExtrinsic>,
	) -> Result<()> {
		trace!("Try to redeem ethereum tx {:?}...", tx.tx_hash);

		// 1. Checking before redeem
		if ethereum2darwinia
			.darwinia
			.verified(tx.block_hash, tx.index)
			.await?
		{
			return Err(BizError::TxRedeemed(tx.tx_hash).into());
		}

		let last_confirmed = ethereum2darwinia.last_confirmed().await?;
		if tx.block >= last_confirmed {
			return Err(BizError::RedeemingBlockLargeThanLastConfirmed(
				tx.tx_hash,
				tx.block,
				last_confirmed,
			)
			.into());
		}

		// 2. Do redeem
		let proof = shadow
			.receipt(&format!("{:?}", tx.enclosed_hash()), last_confirmed)
			.await?;
		let redeem_for = match tx.tx_hash {
			EthereumTransactionHash::Deposit(_) => RedeemFor::Deposit,
			EthereumTransactionHash::Token(_) => RedeemFor::Token,
			EthereumTransactionHash::SetAuthorities(_) => RedeemFor::SetAuthorities,
		};

		let ex = Extrinsic::Redeem(redeem_for, proof, tx);
		let msg = MsgExtrinsic(ex);
		extrinsics_service.send(msg).await?;

		Ok(())
	}
}
