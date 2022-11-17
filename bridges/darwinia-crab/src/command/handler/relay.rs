use bin_s2s::bridge::config::solo_with_solo::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::solo_with_solo::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;
use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-crab", "start bridge darwinia-crab");
    let raw_bridge_config: RawBridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let raw_config_index = raw_bridge_config.index;
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            source: raw_bridge_config.darwinia,
            target: raw_bridge_config.crab,
        },
        relay: raw_bridge_config.relay,
        index: IndexConfig {
            source: BasicSubqueryInfo::new(BridgeName::DarwiniaCrab, raw_config_index.darwinia),
            target: BasicSubqueryInfo::new(BridgeName::DarwiniaCrab, raw_config_index.crab),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
