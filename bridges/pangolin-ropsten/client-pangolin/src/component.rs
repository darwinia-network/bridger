use crate::config::DarwiniaSubxtConfig;
use crate::darwinia::client::Darwinia;

/// Darwinia subxt component
pub struct DarwiniaSubxtComponent;

impl DarwiniaSubxtComponent {
    /// Get darwinia client instance
    pub async fn component(config: DarwiniaSubxtConfig) -> color_eyre::Result<Darwinia> {
        Ok(Darwinia::new(config.endpoint.clone()).await?)
    }
}
