use crate::{Subscan, SubscanConfig};

/// Subscan component
pub struct SubscanComponent;

impl SubscanComponent {
    pub fn component(config: SubscanConfig) -> color_eyre::Result<Subscan> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout.unwrap_or(30)))
            .build()?;
        let subscan = Subscan::new(client, config.endpoint.clone(), config.token.clone());
        Ok(subscan)
    }
}
