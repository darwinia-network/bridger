//! Extrinsics Service
#![allow(missing_docs)]
use std::time::Duration;

use actix::prelude::*;

use crate::error::Result;
use crate::service::redeem::EthereumTransaction;
use crate::service::MsgStop;
use crate::tools;
use primitives::chain::ethereum::{
	EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor,
};
use primitives::runtime::EcdsaMessage;
use std::path::PathBuf;

use darwinia::{Darwinia2Ethereum, Ethereum2Darwinia, FromEthereumAccount, ToEthereumAccount};

#[derive(Clone, Debug)]
pub enum Extrinsic {
	Affirm(EthereumRelayHeaderParcel),
	Redeem(RedeemFor, EthereumReceiptProofThing, EthereumTransaction),
	GuardVote(u64, bool),
	SignAndSendMmrRoot(u32),
	SignAndSendAuthorities(EcdsaMessage),
}

/// MsgSign
#[derive(Clone, Debug)]
pub struct MsgExtrinsic(pub Extrinsic);

impl Message for MsgExtrinsic {
	type Result = Result<()>;
}

/// Extrinsics Service
pub struct ExtrinsicsService {
	/// Ethereum to Darwinia Client
	pub ethereum2darwinia: Option<Ethereum2Darwinia>,
	/// Dawrinia to Ethereum Client
	pub darwinia2ethereum: Option<Darwinia2Ethereum>,
	/// ethereum2darwinia relayer
	pub ethereum2darwinia_relayer: Option<FromEthereumAccount>,
	/// darwinia2ethereum relayer
	pub darwinia2ethereum_relayer: Option<ToEthereumAccount>,

	spec_name: String,
	data_dir: PathBuf,
}

impl Actor for ExtrinsicsService {
	type Context = Context<Self>;

	fn started(&mut self, _: &mut Self::Context) {
		info!("✨ SERVICE STARTED: EX SENDER QUEUE");
	}

	fn stopped(&mut self, _: &mut Self::Context) {
		info!("💤 SERVICE STOPPED: EX SENDER QUEUE")
	}
}

impl Handler<MsgExtrinsic> for ExtrinsicsService {
	type Result = AtomicResponse<Self, Result<()>>;

	fn handle(&mut self, msg: MsgExtrinsic, _: &mut Context<Self>) -> Self::Result {
		let f = ExtrinsicsService::send_extrinsic(
			self.ethereum2darwinia.clone(),
			self.darwinia2ethereum.clone(),
			self.ethereum2darwinia_relayer.clone(),
			self.darwinia2ethereum_relayer.clone(),
			msg.0,
			self.spec_name.clone(),
			self.data_dir.clone(),
		)
		.into_actor(self);

		AtomicResponse::new(Box::pin(f))
	}
}

impl Handler<MsgStop> for ExtrinsicsService {
	type Result = ();

	fn handle(&mut self, _: MsgStop, ctx: &mut Context<Self>) -> Self::Result {
		ctx.stop();
	}
}

impl ExtrinsicsService {
	/// New sign service
	pub fn new(
		ethereum2darwinia: Option<Ethereum2Darwinia>,
		darwinia2ethereum: Option<Darwinia2Ethereum>,
		ethereum2darwinia_relayer: Option<FromEthereumAccount>,
		darwinia2ethereum_relayer: Option<ToEthereumAccount>,
		spec_name: String,
		data_dir: PathBuf,
	) -> ExtrinsicsService {
		ExtrinsicsService {
			ethereum2darwinia,
			darwinia2ethereum,
			ethereum2darwinia_relayer,
			darwinia2ethereum_relayer,
			spec_name,
			data_dir,
		}
	}

	async fn send_extrinsic(
		ethereum2darwinia: Option<Ethereum2Darwinia>,
		darwinia2ethereum: Option<Darwinia2Ethereum>,
		ethereum2darwinia_relayer: Option<FromEthereumAccount>,
		darwinia2ethereum_relayer: Option<ToEthereumAccount>,
		extrinsic: Extrinsic,
		spec_name: String,
		data_dir: PathBuf,
	) -> Result<()> {
		match extrinsic {
			Extrinsic::Affirm(parcel) => {
				let block_number = parcel.header.number;
				if let Some(ethereum2darwinia) = &ethereum2darwinia {
					if let Some(relayer) = &ethereum2darwinia_relayer {
						let ex_hash = ethereum2darwinia.affirm(&relayer, parcel).await?;
						info!(
							"Affirmed ethereum block {} in extrinsic {:?}",
							block_number, ex_hash
						);
					} else {
						info!("cannot affirm without relayer account");
					}
				}
			}

			Extrinsic::Redeem(redeem_for, proof, ethereum_tx) => {
				match redeem_for {
					RedeemFor::SetAuthorities => {
						if let Some(darwinia2ethereum) = &darwinia2ethereum {
							if let Some(relayer) = &darwinia2ethereum_relayer {
								let ex_hash = darwinia2ethereum
									.sync_authorities_change(&relayer, proof)
									.await?;
								info!(
									"Sent ethereum tx {:?} with extrinsic {:?}",
									ethereum_tx.tx_hash, ex_hash
								);
							} else {
								info!("cannot sync authorities changed without relayer account");
							}
						}
					}
					_ => {
						if let Some(ethereum2darwinia) = &ethereum2darwinia {
							if let Some(relayer) = &ethereum2darwinia_relayer {
								let ex_hash = ethereum2darwinia
									.redeem(&relayer, redeem_for, proof)
									.await?;
								info!(
									"Redeemed ethereum tx {:?} with extrinsic {:?}",
									ethereum_tx.tx_hash, ex_hash
								);
							}
						}
					}
				}

				// Update cache
				tools::set_cache(
					data_dir,
					tools::LAST_REDEEMED_CACHE_FILE_NAME,
					ethereum_tx.block,
				)
				.await?;
			}

			Extrinsic::GuardVote(pending_block_number, aye) => {
				if let Some(ethereum2darwinia) = &ethereum2darwinia {
					if let Some(guard) = &ethereum2darwinia_relayer {
						let ex_hash = ethereum2darwinia
							.vote_pending_relay_header_parcel(&guard, pending_block_number, aye)
							.await?;
						if aye {
							info!(
								"Voted to approve: {}, ex hash: {:?}",
								pending_block_number, ex_hash
							);
						} else {
							info!(
								"Voted to reject: {}, ex hash: {:?}",
								pending_block_number, ex_hash
							);
						}
					}
				}
			}

			Extrinsic::SignAndSendMmrRoot(block_number) => {
				if let Some(darwinia2ethereum) = &darwinia2ethereum {
					trace!("Start sign and send mmr_root...");
					if let Some(relayer) = &darwinia2ethereum_relayer {
						let ex_hash = darwinia2ethereum
							.ecdsa_sign_and_submit_signed_mmr_root(
								&relayer,
								spec_name,
								block_number,
							)
							.await?;
						info!(
							"Sign and send mmr root of block {} in extrinsic {:?}",
							block_number, ex_hash
						);
					}
				}
			}

			Extrinsic::SignAndSendAuthorities(message) => {
				trace!("Start sign and send authorities...");
				if let Some(darwinia2ethereum) = &darwinia2ethereum {
					if let Some(relayer) = &darwinia2ethereum_relayer {
						let ex_hash = darwinia2ethereum
							.ecdsa_sign_and_submit_signed_authorities(&relayer, message)
							.await?;
						info!("Sign and send authorities in extrinsic {:?}", ex_hash);
					}
				}
			}
		}

		// Delay for waiting to fininsh
		tokio::time::delay_for(Duration::from_secs(12)).await;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::error::Result;
	use actix::prelude::*;
	use std::time::Duration;

	#[derive(Clone, Copy)]
	struct MyMsg(usize);
	impl Message for MyMsg {
		type Result = Result<usize>;
	}

	struct MyActor;

	impl Actor for MyActor {
		type Context = Context<Self>;
	}

	impl Handler<MyMsg> for MyActor {
		type Result = AtomicResponse<Self, Result<usize>>;

		fn handle(&mut self, msg: MyMsg, _: &mut Self::Context) -> Self::Result {
			AtomicResponse::new(Box::pin(
				async {}
					.into_actor(self)
					.then(move |_, this, _| {
						println!("msg {} processing", msg.0);
						if msg.0 == 888 {
							println!("sleep 5 seconds for {}", msg.0);
							tokio::time::delay_for(Duration::from_secs(5)).into_actor(this)
						} else {
							println!("passing {}", msg.0);
							tokio::time::delay_for(Duration::from_millis(1)).into_actor(this)
						}
					})
					.map(move |_, _, _| {
						println!("at the end of processing {} -----------", msg.0);
						if msg.0 == 666 {
							Err(anyhow::anyhow!("error"))
						} else {
							Ok(msg.0)
						}
					}),
			))
		}
	}

	#[actix_rt::test]
	async fn test_work() {
		let my_actor = MyActor.start();
		if let Ok(r) = my_actor.send(MyMsg(12)).await {
			if let Ok(r2) = r {
				assert_eq!(r2, 12);
			}
		}
	}

	#[actix_rt::test]
	async fn test_error() {
		let my_actor = MyActor.start();
		if let Ok(r) = my_actor.send(MyMsg(666)).await {
			if let Err(e) = r {
				assert_eq!(e.to_string(), "error".to_string());
			}
		}
	}

	#[actix_rt::test]
	async fn test_sending_msgs_in_two_different_coroutines() {
		let my_actor = MyActor.start();
		let my_actor_clone = my_actor.clone();
		tokio::spawn(async move {
			let msg_id = 888;
			if let Ok(r) = my_actor_clone.send(MyMsg(msg_id)).await {
				println!("msg {} sent", msg_id);
				if let Ok(r2) = r {
					assert_eq!(r2, msg_id);
					println!("msg {} processed", msg_id);
				}
			}
		});
		tokio::spawn(async move {
			let msg_id = 12;
			if let Ok(r) = my_actor.send(MyMsg(msg_id)).await {
				println!("msg {} sent", msg_id);
				if let Ok(r2) = r {
					assert_eq!(r2, msg_id);
					println!("msg {} processed", msg_id);
				}
			}
		});
		tokio::time::delay_for(Duration::from_secs(10)).await;
		println!("finished")
	}
}
