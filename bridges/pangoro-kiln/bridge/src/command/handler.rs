use crate::bridge::PangoroKilnServiceManager;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge pangoro-kiln");
    let _manager = PangoroKilnServiceManager::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
