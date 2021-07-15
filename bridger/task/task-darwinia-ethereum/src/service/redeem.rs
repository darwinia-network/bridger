use lifeline::{Bus, Lifeline, Service, Receiver, Sender, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
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
pub struct RedeemService {
    _greet: Lifeline,
}

impl BridgeService for RedeemService {}

impl Service for RedeemService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToRedeemMessage>()?;
        let mut sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        // Components
        let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

        let _greet = Self::try_task(
            &format!("{}-service-redeem", DarwiniaEthereumTask::NAME),
            async move {
                debug!(target: DarwiniaEthereumTask::NAME, "hello redeem");

                // Darwinia client
                let darwinia = component_darwinia_subxt.component().await?;
                let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());

                // Shadow client
                let shadow = Arc::new(component_shadow.component().await?);

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToRedeemMessage::EthereumTransaction(tx) => {
                            RedeemService::redeem(
                                ethereum2darwinia.clone(),
                                shadow.clone(),
                                tx,
                                sender_to_extrinsics.clone(),
                            ).await?;
                        },
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

impl RedeemService {

	async fn redeem(
		ethereum2darwinia: Ethereum2Darwinia,
		shadow: Arc<Shadow>,
		tx: EthereumTransaction,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>
	) -> anyhow::Result<()> {
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
			EthereumTransactionHash::RegisterErc20Token(_) => RedeemFor::RegisterErc20Token,
			EthereumTransactionHash::RedeemErc20Token(_) => RedeemFor::RedeemErc20Token,
		};

		let ex = Extrinsic::Redeem(redeem_for, proof, tx);
        sender_to_extrinsics.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;

		Ok(())
	}
}
