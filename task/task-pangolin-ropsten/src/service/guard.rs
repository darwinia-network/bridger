use std::sync::Arc;
use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use postage::broadcast;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::from_ethereum::{Account as FromEthereumAccount, Ethereum2Darwinia};
use component_shadow::{Shadow, ShadowComponent};

use crate::bus::PangolinRopstenBus;
use crate::config::SubstrateEthereumConfig;
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
                tokio::spawn(async move { run(sender_to_extrinsics).await });

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

#[async_recursion]
async fn run(sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>) {
    if let Err(err) = start(sender_to_extrinsics.clone()).await {
        error!(
            target: PangolinRopstenTask::NAME,
            "guard err {:#?}", err
        );
        sleep(Duration::from_secs(10)).await;
        run(sender_to_extrinsics).await;
    }
}

async fn start(
    sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
) -> anyhow::Result<()> {
    info!(target: PangolinRopstenTask::NAME, "SERVICE RESTARTING...");

    // Components
    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
    let component_shadow = ShadowComponent::restore::<PangolinRopstenTask>()?;

    // Config
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(PangolinRopstenTask::NAME)?;
    let servce_config: SubstrateEthereumConfig = Config::restore(PangolinRopstenTask::NAME)?;

    // Darwinia client & account
    let darwinia = component_darwinia_subxt.component().await?;
    let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
    let account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );
    let guard_account = FromEthereumAccount::new(account);
    let is_tech_comm_member = ethereum2darwinia.is_tech_comm_member(None, &guard_account).await?;

    if is_tech_comm_member {
        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        info!(
            target: PangolinRopstenTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA GUARD"
        );

        loop {
            let ethereum2darwinia_clone = ethereum2darwinia.clone();
            let guard_account_clone = guard_account.clone();
            let shadow_clone = shadow.clone();
            let sender_to_extrinsics_clone = sender_to_extrinsics.clone();

            GuardService::guard(
                ethereum2darwinia_clone,
                guard_account_clone,
                shadow_clone,
                sender_to_extrinsics_clone,
            ).await?;

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
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<()> {
        trace!(
            target: PangolinRopstenTask::NAME,
            "Checking pending headers..."
        );

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        if !pending_headers.is_empty() {
            trace!(
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
                let parcel_from_shadow = shadow.parcel(pending_block_number as usize).await?;
                let ex = if pending_parcel.is_same_as(&parcel_from_shadow) {
                    Extrinsic::GuardVote(pending_block_number, true)
                } else {
                    Extrinsic::GuardVote(pending_block_number, false)
                };
                sender_to_extrinsics
                    .send(ToExtrinsicsMessage::Extrinsic(ex))
                    .await?;
            }
        }

        Ok(())
    }
}
