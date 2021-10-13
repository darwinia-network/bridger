use std::sync::Arc;
use std::time::Duration;

use async_recursion::async_recursion;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use postage::broadcast;
use tokio::time::sleep;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::{Shadow, ShadowComponent};
use support_ethereum::receipt::RedeemFor;

use crate::bus::PangolinRopstenBus;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToRedeemMessage};
use crate::service::{EthereumTransaction, EthereumTransactionHash};
use crate::task::PangolinRopstenTask;

#[derive(Debug)]
pub struct RedeemService {
    _greet: Lifeline,
}

impl BridgeService for RedeemService {}

impl Service for RedeemService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToRedeemMessage>()?;
        let sender_to_redeem = bus.tx::<ToRedeemMessage>()?;
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet = Self::try_task(
            &format!("{}-service-redeem", PangolinRopstenTask::NAME),
            async move {
                let mut helper =
                    RedeemHelper::new(sender_to_extrinsics.clone(), sender_to_redeem.clone()).await;

                while let Some(recv) = rx.recv().await {
                    if let ToRedeemMessage::EthereumTransaction(tx) = recv {
                        if let Err(err) = helper.redeem(tx.clone()).await {
                            error!(target: PangolinRopstenTask::NAME, "redeem err: {:#?}", err);
                            // TODO: Consider the errors more carefully
                            // Maybe a websocket err, so wait 10 secs to reconnect.
                            sleep(Duration::from_secs(10)).await;
                            helper = RedeemHelper::new(
                                sender_to_extrinsics.clone(),
                                sender_to_redeem.clone(),
                            )
                            .await;
                            // for any error when helper recreated, we need put this tx back to the
                            // receive queue
                            let _ = helper.retry(tx);
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
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl RedeemHelper {
    #[async_recursion]
    pub async fn new(
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
        sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
    ) -> Self {
        match RedeemHelper::build(sender_to_extrinsics.clone(), sender_to_redeem.clone()).await {
            Ok(helper) => helper,
            Err(err) => {
                error!(
                    target: PangolinRopstenTask::NAME,
                    "redeem init err: {:#?}", err
                );
                sleep(Duration::from_secs(10)).await;
                RedeemHelper::new(sender_to_extrinsics, sender_to_redeem).await
            }
        }
    }

    async fn build(
        sender_to_extrinsics: postage::broadcast::Sender<ToExtrinsicsMessage>,
        sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
    ) -> anyhow::Result<Self> {
        info!(target: PangolinRopstenTask::NAME, "SERVICE RESTARTING...");

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let component_shadow = ShadowComponent::restore::<PangolinRopstenTask>()?;

        // Darwinia client
        let darwinia = component_darwinia.component().await?;
        let darwinia = Ethereum2Darwinia::new(darwinia.clone());

        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        info!(
            target: PangolinRopstenTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA REDEEM"
        );
        Ok(RedeemHelper {
            sender_to_extrinsics,
            sender_to_redeem,
            darwinia,
            shadow,
        })
    }

    pub async fn retry(&mut self, tx: EthereumTransaction) -> anyhow::Result<()> {
        self.sender_to_redeem
            .send(ToRedeemMessage::EthereumTransaction(tx))
            .await?;
        Ok(())
    }

    pub async fn redeem(&self, tx: EthereumTransaction) -> anyhow::Result<()> {
        RedeemHelper::do_redeem(
            self.darwinia.clone(),
            self.shadow.clone(),
            tx,
            self.sender_to_extrinsics.clone(),
            self.sender_to_redeem.clone(),
        )
        .await?;

        Ok(())
    }

    async fn do_redeem(
        ethereum2darwinia: Ethereum2Darwinia,
        shadow: Arc<Shadow>,
        tx: EthereumTransaction,
        mut sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        mut sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
    ) -> anyhow::Result<()> {
        trace!(
            target: PangolinRopstenTask::NAME,
            "Try to redeem ethereum tx {:?}...",
            tx.tx_hash
        );

        // 1. Checking before redeem
        if ethereum2darwinia
            .darwinia
            .verified(tx.block_hash, tx.index)
            .await?
            || ethereum2darwinia
                .darwinia
                .verified_issuing(tx.block_hash, tx.index)
                .await?
        {
            trace!(
                target: PangolinRopstenTask::NAME,
                "Ethereum tx {:?} redeemed",
                tx.tx_hash
            );
            return Ok(());
        }

        let last_confirmed = ethereum2darwinia.last_confirmed().await?;
        if tx.block >= last_confirmed {
            trace!(
                target: PangolinRopstenTask::NAME,
                "Ethereum tx {:?}'s block {} is large than last confirmed block {}",
                tx.tx_hash,
                tx.block,
                last_confirmed,
            );
            sleep(Duration::from_secs(30)).await;
            sender_to_redeem
                .send(ToRedeemMessage::EthereumTransaction(tx))
                .await?;
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
