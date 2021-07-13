use lifeline::{Bus, Lifeline, Service, Receiver, Sender, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::{
    from_ethereum::{
        Ethereum2Darwinia, Account as FromEthereumAccount
    },
    to_ethereum::{
        Darwinia2Ethereum, Account as ToEthereumAccount
    }
};
use component_shadow::{Shadow, ShadowComponent};

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;
use crate::error::BizError;

use crate::message::{ToRedeemMessage, ToExtrinsicsMessage, Extrinsic};

use std::time::Duration;
use tokio::time::sleep;

use std::sync::Arc;
use postage::broadcast;
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use support_ethereum::receipt::RedeemFor;

#[derive(Debug)]
pub struct ExtrinsicsService {
    _greet: Lifeline,
}

impl BridgeService for ExtrinsicsService {}

impl Service for ExtrinsicsService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let mut rx = bus.rx::<ToExtrinsicsMessage>()?;
        let _greet = Self::try_task(
            &format!("{}-service-extrinsics", DarwiniaEthereumTask::NAME),
            async move {
                debug!(target: DarwiniaEthereumTask::NAME, "hello redeem");

                // let config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
                let darwinia = component_darwinia.component().await?;
                let darwinia_client = Ethereum2Darwinia::new(darwinia);

                let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;
                let shadow = Arc::new(component_shadow.component().await?);

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToExtrinsicsMessage::Extrinsic(ex) => {

                        },
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

impl ExtrinsicsService {
	#[allow(clippy::too_many_arguments)]
	async fn send_extrinsic(
		ethereum2darwinia: Option<Ethereum2Darwinia>,
		darwinia2ethereum: Option<Darwinia2Ethereum>,
		ethereum2darwinia_relayer: Option<FromEthereumAccount>,
		darwinia2ethereum_relayer: Option<ToEthereumAccount>,
		extrinsic: Extrinsic,
		spec_name: String,
	) -> anyhow::Result<()> {
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
					RedeemFor::RegisterErc20Token => {
						// if let Some(ethereum2darwinia) = &ethereum2darwinia {
						// 	if let Some(relayer) = &ethereum2darwinia_relayer {
						// 		let ex_hash =
						// 			ethereum2darwinia.register_erc20(&relayer, proof).await?;
						// 		info!(
						// 			"register erc20 token tx {:?} with extrinsic {:?}",
						// 			ethereum_tx.tx_hash, ex_hash
						// 		);
						// 	}
						// }
					}
					RedeemFor::RedeemErc20Token => {
						// if let Some(ethereum2darwinia) = &ethereum2darwinia {
						// 	if let Some(relayer) = &ethereum2darwinia_relayer {
						// 		let ex_hash =
						// 			ethereum2darwinia.redeem_erc20(&relayer, proof).await?;
						// 		info!(
						// 			"redeem erc20 token tx {:?} with extrinsic {:?}",
						// 			ethereum_tx.tx_hash, ex_hash
						// 		);
						// 	}
						// }
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
                // TODO: use kv database
				// tools::set_cache(
				// 	data_dir,
				// 	tools::LAST_REDEEMED_CACHE_FILE_NAME,
				// 	ethereum_tx.block,
				// )
				// .await?;
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
        sleep(Duration::from_secs(12)).await;

		Ok(())
	}
}
