use support_common::config::{Config, Names};

use crate::bridge::{DarwiniaCrabConfig, DarwiniaCrabTask};

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-crab", "Start bridge darwinia-crab");
    // check config
    let _bridge_config: DarwiniaCrabConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let _task = DarwiniaCrabTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
