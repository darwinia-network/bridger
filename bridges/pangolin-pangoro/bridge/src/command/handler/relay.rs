use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, BridgeTask};

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Start bridge pangolin-pangoro");
    // check config
    let _bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let _task = BridgeTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
