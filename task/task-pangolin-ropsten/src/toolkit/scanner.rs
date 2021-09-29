use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::config::EthereumConfig;
use component_ethereum::web3::Web3Component;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use support_tracker::Tracker;
use support_tracker_evm_log::{EvmClient, EvmLogTracker, LogsHandler};

use crate::config::TaskConfig;
use crate::ethereum::Ethereum;
use crate::task::PangolinRopstenTask;

/// Block Scanner
pub struct RopstenScanner<T: LogsHandler> {
    tracker: Tracker,
    handler: T,
}

impl<T: LogsHandler> RopstenScanner<T> {
    pub fn new(tracker: Tracker, handler: T) -> Self {
        Self { tracker, handler }
    }
}

impl<T: LogsHandler> RopstenScanner<T> {
    pub async fn start(&self) {
        while let Err(e) = self.run().await {
            log::error!(target: PangolinRopstenTask::NAME, "ropsten err {:#?}", e);
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }

    async fn run(&self) -> anyhow::Result<()> {
        log::info!(
            target: PangolinRopstenTask::NAME,
            "ROPSTEN SCAN SERVICE RESTARTING..."
        );

        // Components
        let component_web3 = Web3Component::restore::<PangolinRopstenTask>()?;
        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;

        // Config
        let servce_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;
        let ethereum_config: EthereumConfig = Config::restore(PangolinRopstenTask::NAME)?;

        // Web3 client
        let web3 = component_web3.component().await?;

        // Pangolin client
        let pangolin = component_pangolin_subxt.component().await?;

        let topics_list = component_ethereum::helpers::get_topics_list(ethereum_config);

        log::info!(
            target: PangolinRopstenTask::NAME,
            "✨ SERVICE STARTED: ROPSTEN <> PANGOLIN ROPSTEN SUBSCRIBE"
        );
        let client = EvmClient::new(web3);
        let mut tracker_evm_log = EvmLogTracker::<Ethereum, T>::new(
            client,
            topics_list,
            self.handler.clone(),
            tracker,
            servce_config.interval_ethereum,
        );

        tracker_evm_log.start().await
    }
}
