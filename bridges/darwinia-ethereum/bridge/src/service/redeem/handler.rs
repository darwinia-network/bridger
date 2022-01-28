use std::sync::Arc;

use lifeline::Sender;
use postage::broadcast;

use client_darwinia::component::DarwiniaSubxtComponent;
use client_darwinia::from_ethereum::Ethereum2Darwinia;
use component_shadow::component::ShadowComponent;
use component_shadow::shadow::Shadow;
use component_thegraph_liketh::types::TransactionEntity;
use support_common::config::{Config, Names};

use crate::bridge::{DarwiniaEthereumConfig, Extrinsic, ToExtrinsicsMessage};
use crate::helpers;

pub struct RedeemHandler {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl RedeemHandler {
    pub async fn new(sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>) -> Self {
        let mut times = 0;
        loop {
            times += 1;
            match Self::build(sender_to_extrinsics.clone()).await {
                Ok(v) => return v,
                Err(err) => {
                    tracing::error!(
                        target: "darwinia-ethereum",
                        "[ethereum] Failed to create redeem handler, times: [{}] err: {:#?}",
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
    ) -> color_eyre::Result<Self> {
        tracing::info!(target: "darwinia-ethereum", "SERVICE RESTARTING...");

        let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

        // Darwinia client
        let darwinia = DarwiniaSubxtComponent::component(bridge_config.darwinia).await?;
        let darwinia = Ethereum2Darwinia::new(darwinia);

        // Shadow client
        let shadow = ShadowComponent::component(
            bridge_config.shadow,
            bridge_config.ethereum,
            bridge_config.web3,
        )?;
        let shadow = Arc::new(shadow);

        tracing::info!(
            target: "darwinia-ethereum",
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA REDEEM"
        );
        Ok(RedeemHandler {
            sender_to_extrinsics,
            darwinia,
            shadow,
        })
    }
}

impl RedeemHandler {
    pub async fn redeem(&mut self, tx: TransactionEntity) -> color_eyre::Result<Option<u64>> {
        tracing::trace!(
            target: "darwinia-ethereum",
            "[ethereum] Try to redeem ethereum tx {:?}... in block {}",
            tx.tx_hash,
            tx.block_number
        );

        // 1. Checking before redeem
        if helpers::is_verified(&self.darwinia.darwinia, &tx).await? {
            tracing::trace!(
                target: "darwinia-ethereum",
                "[ethereum] Ethereum tx {:?} redeemed",
                tx.tx_hash
            );
            return Ok(Some(tx.block_number));
        }

        let last_confirmed = self.darwinia.last_confirmed().await?;
        if tx.block_number >= last_confirmed {
            tracing::trace!(
                target: "darwinia-ethereum",
                "[ethereum] Ethereum tx {:?}'s block {} is large than last confirmed block {}",
                tx.tx_hash,
                tx.block_number,
                last_confirmed,
            );
            return Ok(None);
        }

        // 2. Do redeem
        let proof = self.shadow.receipt(&tx.tx_hash, last_confirmed).await?;

        let ex = Extrinsic::Redeem(proof.try_into()?, tx.clone());
        tracing::info!(
            target: "darwinia-ethereum",
            "[ethereum] Redeem extrinsic send to extrinsics service: {:?}. at ethereum block: {}",
            ex,
            tx.block_number
        );
        self.sender_to_extrinsics
            .send(ToExtrinsicsMessage::Extrinsic(ex))
            .await?;

        Ok(Some(tx.block_number))
    }
}
