use bin_s2s::bridge::config::solo_with_solo::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::solo_with_solo::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;
use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "start bridge pangolin-pangoro");
    let raw_bridge_config: RawBridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let raw_config_index = raw_bridge_config.index;
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            source: raw_bridge_config.pangolin,
            target: raw_bridge_config.pangoro,
        },
        relay: raw_bridge_config.relay,
        index: IndexConfig {
            source: BasicSubqueryInfo::new(BridgeName::PangolinPangoro, raw_config_index.pangolin),
            target: BasicSubqueryInfo::new(BridgeName::PangolinPangoro, raw_config_index.pangoro),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
