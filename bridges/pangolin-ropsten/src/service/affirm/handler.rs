use lifeline::Sender;
use std::sync::Arc;

use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::errors::BizError;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::from_ethereum::Ethereum2Darwinia;
use component_shadow::{Shadow, ShadowComponent};
use support_ethereum::block::EthereumHeader;

use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::task::PangolinRopstenTask;

pub struct AffirmHandler {
    microkv: NamespaceMicroKV,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    darwinia: Ethereum2Darwinia,
    shadow: Arc<Shadow>,
}

impl AffirmHandler {
    pub async fn new(
        microkv: NamespaceMicroKV,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> Self {
        loop {
            match AffirmHandler::build(microkv.clone(), sender_to_extrinsics.clone()).await {
                Ok(handler) => return handler,
                Err(err) => {
                    log::error!(
                        target: PangolinRopstenTask::NAME,
                        "Failed to init affirm handler. err: {:#?}",
                        err
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn build(
        microkv: NamespaceMicroKV,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
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
            "✨ SERVICE STARTED: ETHEREUM <> DARWINIA RELAY"
        );
        Ok(AffirmHandler {
            microkv,
            sender_to_extrinsics,
            darwinia,
            shadow,
        })
    }
}

impl AffirmHandler {
    pub async fn affirm(&mut self) -> anyhow::Result<()> {
        let last_confirmed = self.darwinia.last_confirmed().await?;
        let mut relayed = self.microkv.get_as("affirm.relayed")?.unwrap_or(0);
        let target = self.microkv.get_as("affirm.target")?.unwrap_or(0);

        log::trace!(
            target: PangolinRopstenTask::NAME,
            "The last confirmed ethereum block is {}",
            last_confirmed
        );

        if last_confirmed > relayed {
            self.microkv.put("affirm.relayed", &last_confirmed)?;
            relayed = last_confirmed;
        } else {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "The last relayed ethereum block is {}",
                relayed
            );
        }

        if target > relayed {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "Your are affirming ethereum block {}",
                target
            );
            self.do_affirm(target).await?
        } else {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "You do not need to affirm ethereum block {}",
                target
            );
        }

        Ok(())
    }

    pub fn update_target(&self, block_number: u64) -> anyhow::Result<()> {
        let target = self.microkv.get_as("affirm.target")?.unwrap_or(0);

        if block_number > target {
            self.microkv.put("affirm.target", &block_number)?;
        }

        Ok(())
    }

    async fn do_affirm(&mut self, target: u64) -> anyhow::Result<()> {
        // /////////////////////////
        // checking before affirm
        // /////////////////////////
        // 1. pendings check
        let pending_headers = self.darwinia.pending_headers().await?;
        for pending_header in pending_headers {
            let pending_block_number = pending_header.1.header.number;
            if pending_block_number >= target {
                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "The affirming target block {} is in pending",
                    target
                );
                return Ok(());
            }
        }

        // 1. affirmations check
        for (_game_id, game) in self.darwinia.affirmations().await?.iter() {
            for (_round_id, affirmations) in game.iter() {
                if Ethereum2Darwinia::contains(affirmations, target) {
                    log::trace!(
                        target: PangolinRopstenTask::NAME,
                        "The affirming target block {} is in the relayer game",
                        target
                    );
                    return Ok(());
                }
            }
        }

        log::trace!(
            target: PangolinRopstenTask::NAME,
            "Prepare to affirm ethereum block: {}",
            target
        );

        match self.shadow.parcel(target as usize).await {
            Ok(parcel) => {
                if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
                    log::trace!(
                        target: PangolinRopstenTask::NAME,
                        "Shadow service failed to provide parcel for block {}",
                        target
                    );
                    return Ok(());
                }

                // /////////////////////////
                // do affirm
                // /////////////////////////
                let ex = Extrinsic::Affirm(parcel);
                self.sender_to_extrinsics
                    .send(ToExtrinsicsMessage::Extrinsic(ex))
                    .await?
            }
            Err(err) => {
                if let Some(BizError::BlankEthereumMmrRoot(block, msg)) =
                    err.downcast_ref::<BizError>()
                {
                    log::trace!(
                        target: PangolinRopstenTask::NAME,
                        "The parcel of ethereum block {} from Shadow service is blank, the err msg is {}",
                        block,
                        msg
                    );
                    return Ok(());
                }
                return Err(err);
            }
        }

        Ok(())
    }
}
