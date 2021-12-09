use lifeline::Sender;

use bridge_traits::bridge::task::BridgeSand;

use crate::message::{Extrinsic, ToExtrinsicsMessage};
use crate::service::pangolin::types::ScanDataWrapper;
use crate::task::PangolinRopstenTask;

pub struct ScanScheduleMMRRootEvent<'a> {
    data: &'a mut ScanDataWrapper,
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub async fn handle(&mut self) -> anyhow::Result<()> {
        let event = self
            .data
            .subquery
            .query_latest_schedule_mmr_root_event()
            .await?;
        if event.is_none() {
            log::info!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Not have more ScheduleMMRRootEvent"
            );
            return Ok(());
        }
        let latest = event.unwrap();
        if latest.emitted == 1 {
            log::info!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] The latest ScheduleMMRRootEvent is emitted. don't do this again."
            );
            return Ok(());
        } else {
            log::info!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Queried latest ScheduleMMRRootEvent event block is: {} and at block: {}",
                latest.event_block_number,
                latest.at_block_number
            );
        }

        let event_block_number = latest.event_block_number;

        let finalized_block_hash = self.data.darwinia.finalized_head().await?;
        let finalized_block_header_number = match self
            .data
            .darwinia
            .get_block_number_by_hash(finalized_block_hash)
            .await?
        {
            Some(v) => v,
            None => {
                log::warn!(
                    target: PangolinRopstenTask::NAME,
                    "[pangolin] Can not get last block header by finalized block hash: {}",
                    finalized_block_hash
                );
                return Ok(());
            }
        };

        if event_block_number < finalized_block_header_number {
            return Ok(());
        }

        if !self
            .data
            .darwinia2ethereum
            .need_to_sign_mmr_root_of(
                &self.data.account,
                event_block_number,
                Some(finalized_block_header_number),
            )
            .await?
        {
            log::warn!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Don't need to sign mmr root for this event block: {} and at block: {}",
                latest.event_block_number,
                latest.at_block_number
            );
            return Ok(());
        }

        log::info!(
            target: PangolinRopstenTask::NAME,
            "[pangolin] Send sign mmr root for event block: {} and at block: {}",
            latest.event_block_number,
            latest.at_block_number
        );
        let sender = self.data.sender_to_extrinsics_mut();
        let ex = Extrinsic::SignAndSendMmrRoot(latest.event_block_number);
        sender.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;
        Ok(())
    }
}
