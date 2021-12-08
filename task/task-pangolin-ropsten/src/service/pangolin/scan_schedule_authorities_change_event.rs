use std::convert::TryInto;

use lifeline::Sender;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Account as ToEthereumAccount;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_pangolin_subxt::types::EcdsaMessage;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::service::pangolin::types::ScanDataWrapper;
use crate::task::PangolinRopstenTask;

pub struct ScanScheduleAuthoritiesChangeEvent<'a> {
    data: &'a mut ScanDataWrapper,
}

impl<'a> ScanScheduleAuthoritiesChangeEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanScheduleAuthoritiesChangeEvent<'a> {
    pub async fn handle(&mut self) -> anyhow::Result<Option<u32>> {
        let events = self
            .data
            .subquery
            .query_schedule_authorities_change_event(self.data.from, self.data.limit)
            .await?;

        log::debug!(
            target: PangolinRopstenTask::NAME,
            "[pangolin] Track pangolin ScheduleAuthoritiesChangeEvent block: {} and limit: {}",
            self.data.from,
            self.data.limit
        );
        if events.is_empty() {
            log::debug!("[pangolin] Not have more ScheduleAuthoritiesChangeEvent");
            return Ok(None);
        }
        for event in &events {
            let block_number = Some(event.at_block_number);
            let message = event.message.as_slice().try_into()?;
            let need_to_sign = self
                .data
                .darwinia2ethereum
                .is_authority(block_number, &self.data.account)
                .await?
                && self
                    .data
                    .darwinia2ethereum
                    .need_to_sign_authorities(block_number, &self.data.account, message)
                    .await?;

            if !need_to_sign {
                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "[pangolin] The ScheduleAuthoritiesChangeEvent message: {} don't need to sign and send it at block: {}",
                    array_bytes::bytes2hex("0x", message),
                    event.at_block_number
                );
                continue;
            }

            log::trace!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Try sign and send authorities with message: {} at block: {}",
                array_bytes::bytes2hex("0x", message),
                event.at_block_number
            );
            let ex = Extrinsic::SignAndSendAuthorities(message);
            let sender = self.data.sender_to_extrinsics_mut();
            sender.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;
        }
        let latest = events.last().unwrap();
        Ok(Some(latest.at_block_number))
    }
}
