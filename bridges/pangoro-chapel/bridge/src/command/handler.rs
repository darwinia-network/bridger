use crate::bridge::PangoroChapelServiceManager;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge pangoro-chapel(bsc test net)");
    let _manager = PangoroChapelServiceManager::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
