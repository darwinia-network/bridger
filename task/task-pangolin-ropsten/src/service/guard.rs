use std::sync::Arc;
use std::time::Duration;

use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::errors::BizError;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::config::DarwiniaSubxtConfig;
use component_pangolin_subxt::from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia};
use component_shadow::{Shadow, ShadowComponent};

use crate::bus::PangolinRopstenBus;
use crate::config::TaskConfig;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToGuardMessage};
use crate::task::PangolinRopstenTask;

#[derive(Debug)]
pub struct GuardService {
    _greet: Lifeline,
}

impl BridgeService for GuardService {}

impl Service for GuardService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToGuardMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", PangolinRopstenTask::NAME),
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
        log::error!(target: PangolinRopstenTask::NAME, "guard err {:#?}", err);
        sleep(Duration::from_secs(10)).await;
    }
}

async fn run(sender_to_extrinsics: &mut impl Sender<ToExtrinsicsMessage>) -> anyhow::Result<()> {
    log::info!(target: PangolinRopstenTask::NAME, "SERVICE RESTARTING...");

    // Components
    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
    let component_shadow = ShadowComponent::restore::<PangolinRopstenTask>()?;

    // Config
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(PangolinRopstenTask::NAME)?;
    let servce_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;

    // Darwinia client & account
    let darwinia = component_pangolin_subxt.component().await?;
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
        let shadow = Arc::new(component_shadow.component().await?);

        log::info!(
            target: PangolinRopstenTask::NAME,
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
    ) -> anyhow::Result<()> {
        log::trace!(
            target: PangolinRopstenTask::NAME,
            "Checking pending headers..."
        );

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        if !pending_headers.is_empty() {
            log::trace!(
                target: PangolinRopstenTask::NAME,
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
                            log::trace!(
                                target: PangolinRopstenTask::NAME,
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
