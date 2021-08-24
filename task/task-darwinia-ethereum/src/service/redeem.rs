use std::sync::Arc;
use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use postage::broadcast;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::{Shadow, ShadowComponent};
use support_ethereum::receipt::RedeemFor;

use crate::bus::DarwiniaEthereumBus;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToRedeemMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use crate::task::DarwiniaEthereumTask;

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
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-redeem", DarwiniaEthereumTask::NAME),
            async move {
                let mut helper = RedeemHelper::new(sender_to_extrinsics.clone()).await;

                while let Some(recv) = rx.recv().await {
                    if let ToRedeemMessage::EthereumTransaction(tx) = recv {
                        if let Err(err) = helper.redeem(tx).await {
                            error!(target: DarwiniaEthereumTask::NAME, "redeem err: {:#?}", err);
                            // TODO: Consider the errors more carefully
                            // Maybe a websocket err, so wait 10 secs to reconnect.
                            sleep(Duration::from_secs(10)).await;
                            helper = RedeemHelper::new(sender_to_extrinsics.clone()).await;
                            // TODO: Maybe need retry
                        }
                    }
                }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct RedeemHelper {
    sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl RedeemHelper {
    #[async_recursion]
    pub async fn new(
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    ) -> Self {
        match RedeemHelper::build(sender_to_extrinsics.clone()).await {
            Ok(helper) => helper,
            Err(err) => {
                error!(
                    target: DarwiniaEthereumTask::NAME,
                    "redeem init err: {:#?}", err
                );
                sleep(Duration::from_secs(10)).await;
                RedeemHelper::new(sender_to_extrinsics).await
            }
        }
    }

    async fn build(
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<Self> {
        info!(target: DarwiniaEthereumTask::NAME, "SERVICE RESTARTING...");

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;

        // Darwinia client
        let darwinia = component_darwinia.component().await?;
        let darwinia = Ethereum2Darwinia::new(darwinia.clone());

        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        info!(
            target: DarwiniaEthereumTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA REDEEM"
        );
        Ok(RedeemHelper {
            sender_to_extrinsics,
            darwinia,
            shadow,
        })
    }

    pub async fn redeem(&self, tx: EthereumTransaction) -> anyhow::Result<()> {
        RedeemHelper::do_redeem(
            self.darwinia.clone(),
            self.shadow.clone(),
            tx,
            self.sender_to_extrinsics.clone(),
        )
        .await?;

        Ok(())
    }

    async fn do_redeem(
        ethereum2darwinia: Ethereum2Darwinia,
        shadow: Arc<Shadow>,
        tx: EthereumTransaction,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<()> {
        trace!(
            target: DarwiniaEthereumTask::NAME,
            "Try to redeem ethereum tx {:?}...",
            tx.tx_hash
        );

        // 1. Checking before redeem
        if ethereum2darwinia
            .darwinia
            .verified(tx.block_hash, tx.index)
            .await?
        {
            trace!(
                target: DarwiniaEthereumTask::NAME,
                "Ethereum tx {:?} redeemed",
                tx.tx_hash
            );
            return Ok(());
        }

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        if tx.block >= last_confirmed {
            trace!(
                target: DarwiniaEthereumTask::NAME,
                "Ethereum tx {:?}'s block {} is large than last confirmed block {}",
                tx.tx_hash,
                tx.block,
                last_confirmed,
            );
            return Ok(());
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
        sender_to_extrinsics
            .send(ToExtrinsicsMessage::Extrinsic(ex))
            .await?;

        Ok(())
    }
}
