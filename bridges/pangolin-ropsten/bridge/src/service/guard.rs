use std::sync::Arc;
use std::time::Duration;

use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use tokio::time::sleep;

use client_pangolin::account::DarwiniaAccount;
use client_pangolin::component::DarwiniaSubxtComponent;
use client_pangolin::config::DarwiniaSubxtConfig;
use client_pangolin::from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia};
use component_ethereum::errors::BizError;
use component_shadow::{Shadow, ShadowComponent};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::PangolinRopstenBus;
use crate::bridge::PangolinRopstenTask;
use crate::bridge::TaskConfig;
use crate::bridge::{Extrinsic, PangolinRopstenConfig, ToExtrinsicsMessage, ToGuardMessage};

#[derive(Debug)]
pub struct GuardService {
    _greet: Lifeline,
}

impl BridgeService for GuardService {}

impl Service for GuardService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToGuardMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", PangolinRopstenTask::name()),
            async move {
                //
                tokio::spawn(async move { start(sender_to_extrinsics).await });

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToGuardMessage::StartGuard => {}
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(mut sender_to_extrinsics: impl Sender<ToExtrinsicsMessage>) {
    while let Err(err) = run(&mut sender_to_extrinsics).await {
        tracing::error!(target: "pangolin-ropsten", "guard err {:#?}", err);
        sleep(Duration::from_secs(10)).await;
    }
}

async fn run(
    sender_to_extrinsics: &mut impl Sender<ToExtrinsicsMessage>,
) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-ropsten", "SERVICE RESTARTING...");

    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

    // Config
    let config_darwinia: DarwiniaSubxtConfig = bridge_config.darwinia;
    let servce_config: TaskConfig = bridge_config.task;

    // Darwinia client & account
    let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
    let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());

    let account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );
    let guard_account = FromEthereumAccount::new(account);
    let is_tech_comm_member = ethereum2darwinia
        .is_tech_comm_member(None, &guard_account)
        .await?;

    if is_tech_comm_member {
        // Shadow client
        let shadow = ShadowComponent::component(
            bridge_config.shadow,
            bridge_config.ethereum,
            bridge_config.web3,
        )?;
        let shadow = Arc::new(shadow);

        tracing::info!(
            target: "pangolin-ropsten",
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA GUARD"
        );

        loop {
            let ethereum2darwinia_clone = ethereum2darwinia.clone();
            let guard_account_clone = guard_account.clone();
            let shadow_clone = shadow.clone();

            GuardService::guard(
                ethereum2darwinia_clone,
                guard_account_clone,
                shadow_clone,
                sender_to_extrinsics,
            )
            .await?;

            sleep(Duration::from_secs(servce_config.interval_guard)).await;
        }
    }

    Ok(())
}

impl GuardService {
    async fn guard(
        ethereum2darwinia: Ethereum2Darwinia,
        guard_account: FromEthereumAccount,
        shadow: Arc<Shadow>,
        sender_to_extrinsics: &mut impl Sender<ToExtrinsicsMessage>,
    ) -> color_eyre::Result<()> {
        tracing::trace!(
            target: "pangolin-ropsten",
            "Checking pending headers..."
        );

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        if !pending_headers.is_empty() {
            tracing::trace!(
                target: "pangolin-ropsten",
                "pending headers: {:?}",
                pending_headers
                    .clone()
                    .iter()
                    .map(|p| p.1.header.number.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        for pending in pending_headers {
            let pending_parcel = pending.1;
            let voting_state = pending.2;
            let pending_block_number: u64 = pending_parcel.header.number;

            // high than last_confirmed(https://github.com/darwinia-network/bridger/issues/33),
            // and,
            // have not voted
            if pending_block_number > last_confirmed
                && !ethereum2darwinia.has_voted(&guard_account, voting_state)
            {
                match shadow.parcel(pending_block_number as usize).await {
                    Ok(parcel_from_shadow) => {
                        let ex = if pending_parcel.is_same_as(&parcel_from_shadow) {
                            Extrinsic::GuardVote(pending_block_number, true)
                        } else {
                            Extrinsic::GuardVote(pending_block_number, false)
                        };
                        sender_to_extrinsics
                            .send(ToExtrinsicsMessage::Extrinsic(ex))
                            .await?;
                    }
                    Err(err) => {
                        if let Some(BizError::BlankEthereumMmrRoot(block, msg)) =
                            err.downcast_ref::<BizError>()
                        {
                            tracing::trace!(
                                target: "pangolin-ropsten",
                                "The parcel of ethereum block {} from Shadow service is blank, the err msg is {}",
                                block,
                                msg
                            );
                            return Ok(());
                        }
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }
}
