use lifeline::Sender;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

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
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Not have more ScheduleMMRRootEvent"
            );
            return Ok(());
        }
        let latest = event.unwrap();
        if latest.emitted == 1 {
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] The latest ScheduleMMRRootEvent is emitted. don't do this again."
            );
            return Ok(());
        } else {
            log::debug!(
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
            return Ok(());
        }

        let sender = self.data.sender_to_extrinsics_mut();
        let ex = Extrinsic::SignAndSendMmrRoot(latest.event_block_number);
        sender.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;
        Ok(())
    }
}
