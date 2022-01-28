use lifeline::Sender;
use microkv::namespace::NamespaceMicroKV;

use crate::bridge::{Extrinsic, ToExtrinsicsMessage};
use crate::service::pangolin::types::ScanDataWrapper;

pub struct ScanScheduleMMRRootEvent<'a> {
    data: &'a mut ScanDataWrapper,
    microkv: NamespaceMicroKV,
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub fn new(data: &'a mut ScanDataWrapper, microkv: NamespaceMicroKV) -> Self {
        Self { data, microkv }
    }
}

impl<'a> ScanScheduleMMRRootEvent<'a> {
    pub async fn handle(&mut self) -> color_eyre::Result<()> {
        let event = self
            .data
            .subquery
            .query_latest_schedule_mmr_root_event()
            .await?;
        if event.is_none() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] Not have more ScheduleMMRRootEvent"
            );
            return Ok(());
        }
        let latest = event.unwrap();
        if latest.emitted == 1 {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] The latest ScheduleMMRRootEvent is emitted. event block is: {} and at block: {}. don't do this again.",
                latest.event_block_number,
                latest.at_block_number
            );
            return Ok(());
        } else {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] Queried latest ScheduleMMRRootEvent event block is: {} and at block: {}",
                latest.event_block_number,
                latest.at_block_number
            );
        }

        let event_block_number = latest.event_block_number;

        let pangolin = &self.data.pangolin;
        let finalized_block_hash = pangolin.subxt().rpc().finalized_head().await?;
        let block = pangolin
            .subxt()
            .rpc()
            .block(Some(finalized_block_hash))
            .await?;
        let finalized_block_header_number = match block {
            Some(v) => v.block.header.number,
            None => {
                tracing::warn!(
                    target: "pangolin-ropsten",
                    "[pangolin] Can not get last block header by finalized block hash: {}",
                    finalized_block_hash
                );
                return Ok(());
            }
        };

        if finalized_block_header_number < event_block_number {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] The finalized block number ({}) less than event block number ({}). do nothing.",
                finalized_block_header_number,
                event_block_number
            );
            return Ok(());
        }

        let saved_latest: Option<u32> = self.microkv.get_as("latest_mmr_root_sign")?;
        if Some(event_block_number) == saved_latest {
            tracing::info!(
                target: "pangolin-ropsten",
                "[pangolin] This event block number ({}) is already submitted. Don't submit again.",
                event_block_number
            );
            return Ok(());
        }

        if !pangolin
            .ethereum()
            .need_to_sign_mmr_root_of(
                event_block_number,
                Some(finalized_block_header_number),
                pangolin.account().real_account(),
            )
            .await?
        {
            tracing::warn!(
                target: "pangolin-ropsten",
                "[pangolin] Don't need to sign mmr root for this event block: {} and at block: {}",
                latest.event_block_number,
                latest.at_block_number
            );
            return Ok(());
        }

        tracing::info!(
            target: "pangolin-ropsten",
            "[pangolin] Send sign mmr root for event block: {} and at block: {}",
            latest.event_block_number,
            latest.at_block_number
        );
        let sender = self.data.sender_to_extrinsics_mut();
        let ex = Extrinsic::SignAndSendMmrRoot(latest.event_block_number);
        sender.send(ToExtrinsicsMessage::Extrinsic(ex)).await?;

        self.microkv
            .put("latest_mmr_root_sign", &event_block_number)?;
        Ok(())
    }
}
