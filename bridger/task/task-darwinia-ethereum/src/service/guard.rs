use lifeline::{Bus, Lifeline, Service, Receiver, Sender, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::{
    from_ethereum::{
        Ethereum2Darwinia, Account as FromEthereumAccount
    },
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

        // Components
        let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

        // Config
        let config_darwinia: DarwiniaSubxtConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

        let _greet = Self::try_task(
            &format!("{}-service-guard", DarwiniaEthereumTask::NAME),
            async move {
                debug!(target: DarwiniaEthereumTask::NAME, "hello guard");

                // Darwinia client & account
                let darwinia = component_darwinia_subxt.component().await?;
                let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
                let account = DarwiniaAccount::new(config_darwinia.endpoint, config_darwinia.relayer_real_account);
                let guard_account = FromEthereumAccount::new(account);

                // Shadow client
                let shadow = Arc::new(component_shadow.component().await?);

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToGuardMessage::StartGuard => {
                            loop {
                                GuardService::guard(
                                    ethereum2darwinia.clone(),
                                    guard_account.clone(),
                                    shadow.clone(),
                                    sender_to_extrinsics.clone(),
                                ).await?;
                            }

                        },
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

impl GuardService {

    async fn guard(
        ethereum2darwinia: Ethereum2Darwinia,
        guard_account: FromEthereumAccount,
        shadow: Arc<Shadow>,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>
    ) -> anyhow::Result<()> {
        trace!("Checking pending headers...");

        let last_confirmed = ethereum2darwinia.last_confirmed().await.unwrap();
        let pending_headers = ethereum2darwinia.pending_headers().await?;
        if !pending_headers.is_empty() {
            trace!(
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
