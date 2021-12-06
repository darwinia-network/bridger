use bridge_traits::bridge::component::BridgeComponent;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_subquery::SubqueryComponent;
use lifeline::Sender;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;
use crate::service::pangolin::types::ScanDataWrapper;
use crate::task::PangolinRopstenTask;

pub struct ScanScheduleMMRRootEvent<'a> {
    data: &'a ScanDataWrapper,
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub fn new(data: &'a ScanDataWrapper) -> Self {
        Self { data }
    }
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub async fn handle(&mut self) -> anyhow::Result<Option<u32>> {
        let events = self
            .data
            .subquery
            .query_schedule_mmr_root_event(self.data.from, 1)
            .await?;
        log::debug!(
            target: PangolinRopstenTask::NAME,
            "[pangolin] Track pangolin ScheduleMMRRootEvent block: {} and limit: 1",
            self.data.from
        );
        if events.is_empty() {
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Not have more ScheduleMMRRootEvent"
            );
            return Ok(None);
        }

        let latest = events.last().unwrap();
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
                return Ok(None);
            }
        };
        if finalized_block_header_number >= event_block_number
            && self
                .data
                .darwinia2ethereum
                .need_to_sign_mmr_root_of(
                    &self.data.account,
                    event_block_number,
                    Some(finalized_block_header_number),
                )
                .await?
        {
            let sender = self.data.sender_to_extrinsics_mut();
            sender
                .send(ToExtrinsicsMessage::Extrinsic(delayed_ex.clone()))
                .await?;
        }

        Ok(Some(latest.at_block_number))
    }
}
