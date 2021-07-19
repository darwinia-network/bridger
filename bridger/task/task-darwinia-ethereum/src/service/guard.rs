use lifeline::{Bus, Lifeline, Service, Receiver, Sender, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::from_ethereum::{
        Ethereum2Darwinia, Account as FromEthereumAccount
    };
use component_shadow::{Shadow, ShadowComponent};

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;
use crate::error::BizError;

use crate::message::{ToGuardMessage, ToExtrinsicsMessage, Extrinsic};

use std::time::Duration;
use tokio::time::sleep;

use std::sync::Arc;
use postage::broadcast;
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use support_ethereum::receipt::RedeemFor;
use component_darwinia_subxt::account::DarwiniaAccount;
use bridge_traits::bridge::config::Config;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use async_recursion::async_recursion;

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
        let mut sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", DarwiniaEthereumTask::NAME),
            async move {
                info!(target: DarwiniaEthereumTask::NAME, "✨ SERVICE STARTED: ETHEREUM <> DARWINIA GUARD");

                //
                tokio::spawn(async move {
                    run(sender_to_extrinsics).await
                });

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToGuardMessage::StartGuard => {

                        },
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

#[async_recursion]
async fn run(sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>) -> anyhow::Result<()> {
    // Components
    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

    // Config
    let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    // Darwinia client & account
    let darwinia = component_darwinia_subxt.component().await?;
    let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
    let account = DarwiniaAccount::new(config_darwinia.relayer_private_key, config_darwinia.relayer_real_account);
    let guard_account = FromEthereumAccount::new(account);

    // Shadow client
    let shadow = Arc::new(component_shadow.component().await?);

    loop {
        let ethereum2darwinia_clone = ethereum2darwinia.clone();
        let guard_account_clone = guard_account.clone();
        let shadow_clone = shadow.clone();
        let sender_to_extrinsics_clone = sender_to_extrinsics.clone();

        if let Err(err) = GuardService::guard(
            ethereum2darwinia_clone,
            guard_account_clone,
            shadow_clone,
            sender_to_extrinsics_clone
        ).await {
            error!(target: DarwiniaEthereumTask::NAME, "{:#?}", err);
            let err_msg = format!("{:?}", err).to_lowercase();
            if err_msg.contains("restart") {
                break;
            }
        }

        sleep(Duration::from_secs(servce_config.interval_guard)).await;
    }

    sleep(Duration::from_secs(30)).await;
    run(sender_to_extrinsics).await
}

impl GuardService {

    async fn guard(
        ethereum2darwinia: Ethereum2Darwinia,
        guard_account: FromEthereumAccount,
        shadow: Arc<Shadow>,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>
    ) -> anyhow::Result<()> {
        trace!(target: DarwiniaEthereumTask::NAME, "Checking pending headers...");

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
                let parcel_from_shadow = shadow.parcel(pending_block_number as usize).await?;
                let ex = if pending_parcel.is_same_as(&parcel_from_shadow) {
                    Extrinsic::GuardVote(pending_block_number, true)
                } else {
                    Extrinsic::GuardVote(pending_block_number, false)
                };
                sender_to_extrinsics.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;
            }
        }

        Ok(())
    }
}
