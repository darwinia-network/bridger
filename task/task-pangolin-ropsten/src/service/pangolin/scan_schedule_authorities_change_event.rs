use lifeline::Sender;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;
use std::convert::TryInto;

use bridge_traits::bridge::component::BridgeComponent;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Account as ToEthereumAccount;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_pangolin_subxt::types::EcdsaMessage;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::task::PangolinRopstenTask;

pub struct ScanScheduleAuthoritiesChangeEvent {
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    tracker: Tracker,
}

impl ScanScheduleAuthoritiesChangeEvent {
    pub fn new(
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
        microkv: NamespaceMicroKV,
    ) -> Self {
        let tracker = Tracker::new(microkv, "scan.pangolin.schedule-authorities-change");
        Self {
            sender_to_extrinsics,
            tracker,
        }
    }
}

impl ScanScheduleAuthoritiesChangeEvent {
    pub async fn start(&mut self) {
        while let Err(err) = self.run().await {
            log::error!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] An error occurred while processing the ScheduleAuthoritiesChangeEvent: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        // subquery
        let component_subquery = SubqueryComponent::restore::<PangolinRopstenTask>()?;
        let subquery = component_subquery.component().await?;

        // darwinia
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let darwinia = component_pangolin_subxt.component().await?;
        let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
        let account = DarwiniaAccount::new(
            config_darwinia.relayer_private_key,
            config_darwinia.relayer_real_account,
        );
        let account = ToEthereumAccount::new(
            account.clone(),
            config_darwinia.ecdsa_authority_private_key,
            config_web3.endpoint,
        );

        loop {
            let from = self.tracker.current().await?;
            let limit = 10usize;
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Track pangolin ScheduleAuthoritiesChangeEvent block: {} and limit: {}",
                from,
                limit
            );
            let events = subquery
                .query_schedule_authorities_change_event(from as u64, limit as u32)
                .await?;
            for event in events {
                let block_number = Some(event.at_block_number);
                let message = event.message.as_slice().try_into()?;
                let need_to_sign = darwinia2ethereum.is_authority(block_number, &account)
                    && darwinia2ethereum.need_to_sign_authorities(block_number, &account, message);

                if !need_to_sign {
                    log::trace!(
                        target: PangolinRopstenTask::NAME,
                        "[pangolin] The ScheduleAuthoritiesChangeEvent message: {} don't need to sign and send it at block: {:?}",
                        array_bytes::bytes2hex("0x", message),
                        block_number
                    );
                    continue;
                }

                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "[pangolin] Try sign and send authorities with message: {} at block: {:?}",
                    array_bytes::bytes2hex("0x", message),
                    block_number
                );
                let ex = Extrinsic::SignAndSendAuthorities(message);
                self.sender_to_extrinsics
                    .send(ToExtrinsicsMessage::Extrinsic(ex))
                    .await?;
            }
        }
    }
}
