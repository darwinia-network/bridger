use std::convert::TryInto;

use lifeline::Sender;

use crate::bridge::{Extrinsic, ToExtrinsicsMessage};
use crate::service::pangolin::types::ScanDataWrapper;

pub struct ScanScheduleAuthoritiesChangeEvent<'a> {
    data: &'a mut ScanDataWrapper,
}

impl<'a> ScanScheduleAuthoritiesChangeEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanScheduleAuthoritiesChangeEvent<'a> {
    pub async fn handle(&mut self) -> color_eyre::Result<Option<u32>> {
        let events = self
            .data
            .subquery
            .query_schedule_authorities_change_event(self.data.from, self.data.limit)
            .await?;

        tracing::debug!(
            target: "pangolin-ropsten",
            "[pangolin] [schedule-authorities-change] Track pangolin ScheduleAuthoritiesChangeEvent block: {} and limit: {}",
            self.data.from,
            self.data.limit
        );
        if events.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [schedule-authorities-change] Not have more ScheduleAuthoritiesChangeEvent"
            );
            return Ok(None);
        }
        for event in &events {
            let block_number = Some(event.at_block_number);
            let message = event.message.as_slice().try_into()?;
            let pangolin = &self.data.pangolin;
            let real_account = pangolin.account().real_account();
            let need_to_sign = pangolin
                .ethereum()
                .is_authority(block_number, real_account)
                .await?
                && pangolin
                    .ethereum()
                    .need_to_sign_authorities(block_number, real_account, message)
                    .await?;

            if !need_to_sign {
                tracing::trace!(
                    target: "pangolin-ropsten",
                    "[pangolin] [schedule-authorities-change] The ScheduleAuthoritiesChangeEvent message: {} don't need to sign and send it at block: {}",
                    array_bytes::bytes2hex("0x", message),
                    event.at_block_number
                );
                continue;
            }

            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] [schedule-authorities-change] Try sign and send authorities with message: {} at block: {}",
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
