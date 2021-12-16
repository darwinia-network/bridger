use std::sync::Arc;

use lifeline::Sender;
use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::{Shadow, ShadowComponent};
use component_thegraph_liketh::types::TransactionEntity;

use crate::helpers;
use crate::message::{Extrinsic, ToExtrinsicsMessage, ToRedeemMessage};
use crate::task::PangolinRopstenTask;

pub struct RedeemHandler {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl RedeemHandler {
    pub async fn new(
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    ) -> Self {
        let mut times = 0;
        loop {
            times += 1;
            match Self::build(sender_to_extrinsics.clone(), sender_to_redeem.clone()).await {
                Ok(v) => return v,
                Err(err) => {
                    log::error!(
                        target: PangolinRopstenTask::NAME,
                        "[ropsten] Failed to create redeem handler, times: [{}] err: {:#?}",
                        times,
                        err
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn build(
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    ) -> anyhow::Result<Self> {
        log::info!(target: PangolinRopstenTask::NAME, "SERVICE RESTARTING...");

        // Components
        let component_darwinia = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let component_shadow = ShadowComponent::restore::<PangolinRopstenTask>()?;

        // Darwinia client
        let darwinia = component_darwinia.component().await?;
        let darwinia = Ethereum2Darwinia::new(darwinia.clone());

        // Shadow client
        let shadow = Arc::new(component_shadow.component().await?);

        log::info!(
            target: PangolinRopstenTask::NAME,
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA REDEEM"
        );
        Ok(RedeemHandler {
            sender_to_extrinsics,
            sender_to_redeem,
            darwinia,
            shadow,
        })
    }
}

impl RedeemHandler {
    pub async fn redeem(&mut self, tx: TransactionEntity) -> anyhow::Result<Option<u64>> {
        log::trace!(
            target: PangolinRopstenTask::NAME,
            "[ropsten] Try to redeem ethereum tx {:?}... in block {}",
            tx.tx_hash,
            tx.block_number
        );

        // 1. Checking before redeem
        if helpers::is_verified(&self.darwinia.darwinia, &tx).await? {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "[ropsten] Ethereum tx {:?} redeemed",
                tx.tx_hash
            );
            return Ok(Some(tx.block_number));
        }

        let last_confirmed = self.darwinia.last_confirmed().await?;
        if tx.block_number >= last_confirmed {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "[ropsten] Ethereum tx {:?}'s block {} is large than last confirmed block {}",
                tx.tx_hash,
                tx.block_number,
                last_confirmed,
            );
            // tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            // self.sender_to_redeem
            //     .send(ToRedeemMessage::EthereumTransaction(tx))
            //     .await?;
            return Ok(None);
        }

        // 2. Do redeem
        let proof = self.shadow.receipt(&tx.tx_hash, last_confirmed).await?;

        let ex = Extrinsic::Redeem(proof, tx.clone());
        log::info!(
            target: PangolinRopstenTask::NAME,
            "[ropsten] Redeem extrinsic send to extrinsics service: {:?}. at ropsten block: {}",
            ex,
            tx.block_number
        );
        self.sender_to_extrinsics
            .send(ToExtrinsicsMessage::Extrinsic(ex))
            .await?;

        Ok(Some(tx.block_number))
    }
}
