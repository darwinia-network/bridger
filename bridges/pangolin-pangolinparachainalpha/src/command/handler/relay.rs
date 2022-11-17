use bin_s2s::bridge::config::solo_with_para::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::solo_with_para::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;
use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangolinparachain", "start bridge pangolin-pangolinparachain");
    let raw_bridge_config: RawBridgeConfig =
        Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let raw_config_index = raw_bridge_config.index;
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            solo: raw_bridge_config.pangolin,
            para: raw_bridge_config.pangolin_parachain_alpha,
            relay: raw_bridge_config.moonbase,
        },
        relay: raw_bridge_config.relay,
        index: IndexConfig {
            solo: BasicSubqueryInfo::new(
                BridgeName::PangolinPangolinParachainAlpha,
                raw_config_index.pangolin,
            ),
            para: BasicSubqueryInfo::new(
                BridgeName::PangolinPangolinParachainAlpha,
                raw_config_index.pangolin_parachain_alpha,
            ),
            relay: BasicSubqueryInfo::new(
                BridgeName::PangolinPangolinParachainAlpha,
                raw_config_index.moonbase,
            ),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
