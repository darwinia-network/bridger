use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use microkv::MicroKV;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::{
    from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia},
    to_ethereum::{Account as ToEthereumAccount, Darwinia2Ethereum},
};
use component_ethereum::config::Web3Config;
use component_state::state::BridgeState;
use support_ethereum::receipt::RedeemFor;

use crate::bus::DarwiniaEthereumBus;
use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::task::DarwiniaEthereumTask;

#[derive(Debug)]
pub struct ExtrinsicsService {
    _greet: Lifeline,
}

impl BridgeService for ExtrinsicsService {}

impl Service for ExtrinsicsService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToExtrinsicsMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-extrinsics", DarwiniaEthereumTask::NAME),
            async move {
                let mut helper = ExtrinsicsHelper::new(state.clone()).await;

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToExtrinsicsMessage::Extrinsic(ex) => {
                            if let Err(err) = helper.send_extrinsic(ex).await {
                                error!(
                                    target: DarwiniaEthereumTask::NAME,
                                    "extrinsics err: {:#?}", err
                                );

                                // TODO: Consider the errors more carefully
                                // TODO: Maybe need retry

                                // Maybe a websocket err, so wait 10 secs to reconnect.
                                sleep(Duration::from_secs(10)).await;

                                helper = ExtrinsicsHelper::new(state.clone()).await;
                            }
                        }
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct ExtrinsicsHelper {
    state: BridgeState,
    ethereum2darwinia: Ethereum2Darwinia,
    darwinia2ethereum: Darwinia2Ethereum,
    darwinia2ethereum_relayer: ToEthereumAccount,
    ethereum2darwinia_relayer: FromEthereumAccount,
    spec_name: String,
}

impl ExtrinsicsHelper {
    #[async_recursion]
    pub async fn new(state: BridgeState) -> Self {
        match ExtrinsicsHelper::build(state.clone()).await {
            Ok(helper) => helper,
            Err(err) => {
                error!(
                    target: DarwiniaEthereumTask::NAME,
                    "extrinsics init err: {:#?}", err
                );
                sleep(Duration::from_secs(10)).await;
                ExtrinsicsHelper::new(state).await
            }
        }
    }

    async fn build(state: BridgeState) -> anyhow::Result<Self> {
        info!(target: DarwiniaEthereumTask::NAME, "SERVICE RESTARTING...");

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;

        // Config
        let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
        let config_web3: Web3Config = Config::restore(DarwiniaEthereumTask::NAME)?;

        // Darwinia client & accounts
        let darwinia = component_darwinia.component().await?;
        let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
        let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
        let account = DarwiniaAccount::new(
            config_darwinia.relayer_private_key,
            config_darwinia.relayer_real_account,
        );
        let darwinia2ethereum_relayer = ToEthereumAccount::new(
            account.clone(),
            config_darwinia.ecdsa_authority_private_key,
            config_web3.endpoint,
        );
        let ethereum2darwinia_relayer = FromEthereumAccount::new(account);

        let spec_name = darwinia.runtime_version().await?;

        info!(
            target: DarwiniaEthereumTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA EXTRINSICS"
        );

        Ok(ExtrinsicsHelper {
            state,
            ethereum2darwinia,
            darwinia2ethereum,
            darwinia2ethereum_relayer,
            ethereum2darwinia_relayer,
            spec_name,
        })
    }

    async fn send_extrinsic(&self, ex: Extrinsic) -> anyhow::Result<()> {
        let microkv = self.state.microkv();
        do_send_extrinsic(
            microkv,
            Some(self.ethereum2darwinia.clone()),
            Some(self.darwinia2ethereum.clone()),
            Some(self.ethereum2darwinia_relayer.clone()),
            Some(self.darwinia2ethereum_relayer.clone()),
            ex,
            self.spec_name.clone(),
        )
        .await
    }
}

#[allow(clippy::too_many_arguments)]
async fn do_send_extrinsic(
    microkv: &MicroKV,
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
                    let ex_hash = ethereum2darwinia.affirm(relayer, parcel).await?;
                    info!(
                        target: DarwiniaEthereumTask::NAME,
                        "Affirmed ethereum block {} in extrinsic {:?}", block_number, ex_hash
                    );
                } else {
                    info!(
                        target: DarwiniaEthereumTask::NAME,
                        "cannot affirm without relayer account"
                    );
                }
            }
        }

        Extrinsic::Redeem(redeem_for, proof, ethereum_tx) => {
            match redeem_for {
                RedeemFor::SetAuthorities => {
                    if let Some(darwinia2ethereum) = &darwinia2ethereum {
                        if let Some(relayer) = &darwinia2ethereum_relayer {
                            let ex_hash = darwinia2ethereum
                                .sync_authorities_change(relayer, proof)
                                .await?;
                            info!(
                                target: DarwiniaEthereumTask::NAME,
                                "Sent ethereum tx {:?} with extrinsic {:?}",
                                ethereum_tx.tx_hash,
                                ex_hash
                            );
                        } else {
                            info!(
                                target: DarwiniaEthereumTask::NAME,
                                "cannot sync authorities changed without relayer account"
                            );
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
                                .redeem(relayer, redeem_for, proof)
                                .await?;
                            info!(
                                target: DarwiniaEthereumTask::NAME,
                                "Redeemed ethereum tx {:?} with extrinsic {:?}",
                                ethereum_tx.tx_hash,
                                ex_hash
                            );
                        }
                    }
                }
            }

            // Update cache
            microkv.put("last-redeemed", &ethereum_tx.block)?;
        }

        Extrinsic::GuardVote(pending_block_number, aye) => {
            if let Some(ethereum2darwinia) = &ethereum2darwinia {
                if let Some(guard) = &ethereum2darwinia_relayer {
                    let ex_hash = ethereum2darwinia
                        .vote_pending_relay_header_parcel(guard, pending_block_number, aye)
                        .await?;
                    if aye {
                        info!(
                            target: DarwiniaEthereumTask::NAME,
                            "Voted to approve: {}, ex hash: {:?}", pending_block_number, ex_hash
                        );
                    } else {
                        info!(
                            target: DarwiniaEthereumTask::NAME,
                            "Voted to reject: {}, ex hash: {:?}", pending_block_number, ex_hash
                        );
                    }
                }
            }
        }

        Extrinsic::SignAndSendMmrRoot(block_number) => {
            if let Some(darwinia2ethereum) = &darwinia2ethereum {
                trace!(
                    target: DarwiniaEthereumTask::NAME,
                    "Start sign and send mmr_root..."
                );
                if let Some(relayer) = &darwinia2ethereum_relayer {
                    let ex_hash = darwinia2ethereum
                        .ecdsa_sign_and_submit_signed_mmr_root(relayer, spec_name, block_number)
                        .await?;
                    info!(
                        target: DarwiniaEthereumTask::NAME,
                        "Sign and send mmr root of block {} in extrinsic {:?}",
                        block_number,
                        ex_hash
                    );
                }
            }
        }

        Extrinsic::SignAndSendAuthorities(message) => {
            trace!(
                target: DarwiniaEthereumTask::NAME,
                "Start sign and send authorities..."
            );
            if let Some(darwinia2ethereum) = &darwinia2ethereum {
                if let Some(relayer) = &darwinia2ethereum_relayer {
                    let ex_hash = darwinia2ethereum
                        .ecdsa_sign_and_submit_signed_authorities(relayer, message)
                        .await?;
                    info!(
                        target: DarwiniaEthereumTask::NAME,
                        "Sign and send authorities in extrinsic {:?}", ex_hash
                    );
                }
            }
        }
    }

    // Delay for waiting to fininsh
    sleep(Duration::from_secs(12)).await;

    Ok(())
}
