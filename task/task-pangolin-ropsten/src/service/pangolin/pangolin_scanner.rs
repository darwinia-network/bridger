use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::ethereum::EthereumComponent;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Account as ToEthereumAccount;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_state::state::BridgeState;
use component_subquery::subquery::Subquery;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;
use crate::service::pangolin::scan_authorities_change_signed_event::ScanAuthoritiesChangeSignedEvent;
use crate::service::pangolin::scan_schedule_authorities_change_event::ScanScheduleAuthoritiesChangeEvent;
use crate::service::pangolin::scan_schedule_mmr_root_event::ScanScheduleMMRRootEvent;
use crate::service::pangolin::types::ScanDataWrapper;
use crate::task::PangolinRopstenTask;

pub struct PangolinScanner;

impl PangolinScanner {
    pub async fn start(
        &self,
        tracker: Tracker,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) {
        while let Err(err) = self
            .run(tracker.clone(), sender_to_extrinsics.clone())
            .await
        {
            log::error!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] An error occurred while processing the extrinsics: {:?}",
                err
            );
            // Prevent too fast refresh errors
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    async fn run(
        &self,
        tracker: Tracker,
        sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
    ) -> anyhow::Result<()> {
        // subquery
        let component_subquery = SubqueryComponent::restore::<PangolinRopstenTask>()?;
        let subquery = component_subquery.component().await?;

        // darwinia
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        let darwinia = component_pangolin_subxt.component().await?;

        // ethereum
        let component_ethereum = EthereumComponent::restore::<PangolinRopstenTask>()?;
        let ethereum = component_ethereum.component().await?;

        let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());
        let account = account1;
        let account = ToEthereumAccount::new(
            account.clone(),
            config_darwinia.ecdsa_authority_private_key,
            config_web3.endpoint,
        );
        let mut wrapper = ScanDataWrapper {
            from: 0,
            limit: 0,
            sender_to_extrinsics,
            subquery,
            darwinia,
            ethereum,
            darwinia2ethereum,
            account,
        };

        loop {
            let from = tracker.current().await?;
            let limit = 10u32;
            log::debug!(
                target: PangolinRopstenTask::NAME,
                "[pangolin] Track pangolin scan block: {} and limit: {}",
                from,
                limit
            );
            wrapper.from = from as u64;
            wrapper.limit = limit;
            let mut scan_authorities_change_signed_event =
                ScanAuthoritiesChangeSignedEvent::new(&wrapper);
            let max_0 = scan_authorities_change_signed_event.handle().await?;

            let mut scan_schedule_authorities_change_event =
                ScanScheduleAuthoritiesChangeEvent::new(&wrapper);
            let max_1 = scan_schedule_authorities_change_event.handle().await?;

            let mut scan_schedule_mmr_root_event = ScanScheduleMMRRootEvent::new(&wrapper);
            let max_2 = scan_schedule_mmr_root_event.handle().await?;

            let max_block_number = std::cmp::max(max_0, max_1);
            let max_block_number = std::cmp::max(max_block_number, max_2);
            if let Some(block_number) = max_block_number {
                tracker.finish(block_number as usize)?;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
}
