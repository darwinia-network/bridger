use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::account::DarwiniaAccount;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::config::DarwiniaSubxtConfig;
use component_darwinia_subxt::to_ethereum::Account as ToEthereumAccount;
use component_darwinia_subxt::to_ethereum::Darwinia2Ethereum;
use component_ethereum::config::Web3Config;
use component_ethereum::ethereum::EthereumComponent;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::message::ToExtrinsicsMessage;
use crate::service::darwinia::scan_authorities_change_signed_event::ScanAuthoritiesChangeSignedEvent;
use crate::service::darwinia::scan_schedule_authorities_change_event::ScanScheduleAuthoritiesChangeEvent;
use crate::service::darwinia::scan_schedule_mmr_root_event::ScanScheduleMMRRootEvent;
use crate::service::darwinia::types::ScanDataWrapper;
use crate::task::DarwiniaEthereumTask;

pub struct DarwiniaScanner;

impl DarwiniaScanner {
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
                target: DarwiniaEthereumTask::NAME,
                "[darwinia] An error occurred while processing the extrinsics: {:?}",
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
        let component_subquery = SubqueryComponent::restore::<DarwiniaEthereumTask>()?;
        let subquery = component_subquery.component().await?;

        // darwinia
        let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
        let darwinia = component_darwinia_subxt.component().await?;

        // ethereum
        let component_ethereum = EthereumComponent::restore::<DarwiniaEthereumTask>()?;
        let ethereum = component_ethereum.component().await?;

        let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());

        // config
        let config_darwinia: DarwiniaSubxtConfig =
            Config::restore_unwrap(DarwiniaEthereumTask::NAME)?;
        let config_web3: Web3Config = Config::restore_unwrap(DarwiniaEthereumTask::NAME)?;

        let account = DarwiniaAccount::new(
            config_darwinia.relayer_private_key,
            config_darwinia.relayer_real_account,
        );
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
            log::info!(
                target: DarwiniaEthereumTask::NAME,
                "[darwinia] Track darwinia scan block: {} and limit: {}",
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

            let mut scan_schedule_mmr_root_event = ScanScheduleMMRRootEvent::new(&mut wrapper);
            scan_schedule_mmr_root_event.handle().await?;

            let max_block_number = std::cmp::max(max_0, max_1);
            if let Some(block_number) = max_block_number {
                tracker.finish(block_number as usize)?;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }
}
