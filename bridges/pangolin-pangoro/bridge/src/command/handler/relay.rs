use support_common::config::{Config, Names};

use crate::bridge::{PangolinPangoroConfig, PangolinPangoroTask};

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Start bridge pangolin-pangoro");
    // check config
    let _bridge_config: PangolinPangoroConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let _task = PangolinPangoroTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
