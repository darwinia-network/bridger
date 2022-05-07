use lifeline::Sender;
use postage::broadcast;

use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use shadow_liketh::component::ShadowComponent;
use shadow_liketh::shadow::Shadow;
use shadow_liketh::types::BridgeName;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use thegraph_liketh::types::TransactionEntity;

use crate::bridge::{Extrinsic, PangolinRopstenConfig, ToExtrinsicsMessage};

pub struct RedeemHandler {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    client: PangolinClient,
    shadow: Shadow,
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
                        target: "pangolin-ropsten",
                        "[ropsten] [redeem] Failed to create redeem handler, times: [{}] err: {:#?}",
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
        tracing::info!(target: "pangolin-ropsten", "[ropsten] [redeem] RECREATE SCAN REDEEM HANDLER...");

        let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

        // Darwinia client
        let client = PangolinClientComponent::component(bridge_config.darwinia).await?;

        // Shadow client
        let shadow = ShadowComponent::component(
            bridge_config.shadow,
            bridge_config.ethereum,
            bridge_config.web3,
            BridgeName::PangolinRopsten,
        )?;

        tracing::info!(
            target: "pangolin-ropsten",
            "[ropsten] [redeem] ✨ REDEEM HANDLER CREATED: ROPSTEN <> PANGOLIN REDEEM"
        );
        Ok(RedeemHandler {
            sender_to_extrinsics,
            client,
            shadow,
        })
    }
}

impl RedeemHandler {
    pub async fn redeem(&mut self, tx: TransactionEntity) -> color_eyre::Result<Option<u64>> {
        tracing::trace!(
            target: "pangolin-ropsten",
            "[ropsten] [redeem] Try to redeem ethereum tx {:?}... in block {}",
            tx.tx_hash,
            tx.block_number
        );

        // 1. Checking before redeem
        let tx_hash = array_bytes::hex2bytes(&tx.block_hash).map_err(|_e| {
            BridgerError::Hex(format!(
                "Failed to convert hex({}) to bytes.",
                &tx.block_hash
            ))
        })?;
        let tx_index = tx.tx_index;
        if self
            .client
            .ethereum()
            .is_verified(&tx_hash, tx_index)
            .await?
        {
            tracing::trace!(
                target: "pangolin-ropsten",
                "[ropsten] [redeem] Ethereum tx {:?} redeemed",
                tx.tx_hash
            );
            return Ok(Some(tx.block_number));
        }

        let last_confirmed = self.client.ethereum().last_confirmed().await?;
        if tx.block_number >= last_confirmed {
            tracing::trace!(
                target: "pangolin-ropsten",
                "[ropsten] [redeem] Ethereum tx {:?}'s block {} is large than last confirmed block {}",
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
            target: "pangolin-ropsten",
            "[ropsten] [redeem] Redeem extrinsic send to extrinsics service: {:?}. at ropsten block: {}",
            ex,
            tx.block_number
        );
        self.sender_to_extrinsics
            .send(ToExtrinsicsMessage::Extrinsic(ex))
            .await?;

        Ok(Some(tx.block_number))
    }
}
