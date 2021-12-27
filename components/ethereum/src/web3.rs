use web3::transports::Http;
use web3::Web3;

/// Web3 provider, get web3 instance
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Web3Provider {
    /// Then endpoint for web3
    pub endpoint: String,
}

impl Web3Provider {
    /// Get web3 instance
    pub fn component(&self) -> color_eyre::Result<Web3<Http>> {
        Ok(Web3::new(Http::new(&self.config.endpoint).map_err(
            |e| StandardError::NewHttpError(self.config.endpoint.clone(), e.to_string()),
        )?))
    }
}
