use std::sync::Arc;
use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
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
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::TaskConfig;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToGuardMessage};
use crate::task::DarwiniaEthereumTask;

use component_ethereum::error::BizError;
use component_ethereum::error::ComponentEthereumError;

#[derive(Debug)]
pub struct GuardService {
    _greet: Lifeline,
}

impl BridgeService for GuardService {}

impl Service for GuardService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToGuardMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", DarwiniaEthereumTask::NAME),
            async move {
                //
                tokio::spawn(async move { run(state, sender_to_extrinsics).await });

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
async fn run(
    state: BridgeState,
    sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
) {
    if let Err(err) = start(state.clone(), sender_to_extrinsics.clone()).await {
        error!(
            target: DarwiniaEthereumTask::NAME,
            "guard err {:#?}", err
        );
        sleep(Duration::from_secs(10)).await;
        run(state, sender_to_extrinsics).await;
    }
}

async fn start(
    state: BridgeState,
    sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
) -> anyhow::Result<()> {
    info!(target: DarwiniaEthereumTask::NAME, "SERVICE RESTARTING...");

    // Components
    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

    // Config
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
    let servce_config: TaskConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    // Darwinia client & account
    let darwinia = component_darwinia_subxt.component().await?;
    let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
    let account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key_decrypt(
            state.get_task_config_password_unwrap_or_default(DarwiniaEthereumTask::NAME)?,
        )?,
        config_darwinia.relayer_real_account,
    );
    let guard_account = FromEthereumAccount::new(account);
    let is_tech_comm_member = ethereum2darwinia.is_tech_comm_member(None, &guard_account).await?;

    if is_tech_comm_member {
        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        info!(
            target: DarwiniaEthereumTask::NAME,
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
    pub async fn guard<S>(
        ethereum2darwinia: Ethereum2Darwinia,
        guard_account: FromEthereumAccount,
        shadow: Arc<Shadow>,
        mut sender_to_extrinsics: S,
    ) -> anyhow::Result<()>
    where
        S: lifeline::Sender<ToExtrinsicsMessage>,
    {
        trace!(
            target: DarwiniaEthereumTask::NAME,
            "Checking pending headers..."
        );

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        if !pending_headers.is_empty() {
            trace!(
                target: DarwiniaEthereumTask::NAME,
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
                    },
                    Err(ComponentEthereumError::Biz(BizError::BlankEthereumMmrRoot(block, msg))) => {
                        trace!(
                            target: DarwiniaEthereumTask::NAME,
                            "The parcel of ethereum block {} from Shadow service is blank, the err msg is {}",
                            block,
                            msg
                        );
                    },
                    Err(err) => {
                        return Err(err.into());
                    }

                }
            }
        }

        Ok(())
    }
}
