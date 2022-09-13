use client_darwinia::component::DarwiniaClientComponent;
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use component_ethereum::ethereum::EthereumComponent;
use subquery::types::BridgeName;
use subquery::SubqueryComponent;
use support_common::config::{Config, Names};
use support_tracker::Tracker;

use crate::bridge::DarwiniaEthereumConfig;
use crate::bridge::ToExtrinsicsMessage;
use crate::service::darwinia::scan_authorities_change_signed_event::ScanAuthoritiesChangeSignedEvent;
use crate::service::darwinia::scan_schedule_authorities_change_event::ScanScheduleAuthoritiesChangeEvent;
use crate::service::darwinia::scan_schedule_mmr_root_event::ScanScheduleMMRRootEvent;
use crate::service::darwinia::types::ScanDataWrapper;

pub struct DarwiniaScanner;

impl DarwiniaScanner {
    pub async fn start(
        &self,
        microkv: NamespaceMicroKV,
        tracker: Tracker,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) {
        while let Err(err) = self
            .run(
                microkv.clone(),
                tracker.clone(),
                sender_to_extrinsics.clone(),
            )
            .await
        {
            tracing::error!(
                target: "darwinia-ethereum",
                "[darwinia] [scanner] An error occurred while processing the extrinsics: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run(
        &self,
        microkv: NamespaceMicroKV,
        tracker: Tracker,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> color_eyre::Result<()> {
        let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

        // config
        let config_darwinia = bridge_config.darwinia;
        let config_web3 = bridge_config.web3;

        // subquery
        let subquery =
            SubqueryComponent::component(bridge_config.subquery, BridgeName::DarwiniaEthereum);

        // ethereum
        let ethereum = EthereumComponent::component(bridge_config.ethereum, config_web3.clone())?;

        // darwinia client
        let client = DarwiniaClientComponent::component(config_darwinia).await?;

        let mut wrapper = ScanDataWrapper {
            from: 0,
            limit: 0,
            sender_to_extrinsics,
            subquery,
            ethereum,
            darwinia: client,
        };

        loop {
            let from = tracker.current().await?;
            let limit = 10u32;
            tracing::info!(
                target: "darwinia-ethereum",
                "[darwinia] [scanner] Track darwinia scan block: {} and limit: {}",
                from,
                limit
            );
            wrapper.from = from as u64;
            wrapper.limit = limit;
            let mut scan_authorities_change_signed_event =
                ScanAuthoritiesChangeSignedEvent::new(&mut wrapper);
            let max_0 = scan_authorities_change_signed_event.handle().await?;

            let mut scan_schedule_authorities_change_event =
                ScanScheduleAuthoritiesChangeEvent::new(&mut wrapper);
            let max_1 = scan_schedule_authorities_change_event.handle().await?;

            let mut scan_schedule_mmr_root_event =
                ScanScheduleMMRRootEvent::new(&mut wrapper, microkv.clone());
            scan_schedule_mmr_root_event.handle().await?;

            let max_block_number = std::cmp::max(max_0, max_1);
            if let Some(block_number) = max_block_number {
                tracker.finish(block_number as usize)?;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
}
