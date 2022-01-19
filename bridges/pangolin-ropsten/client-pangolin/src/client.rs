use subxt::Client;

use crate::config::PangolinSubxtConfig;
use crate::ethereum::{FromEthereumApi, ToEthereumApi};

/// Pangolin client
#[derive(Clone)]
pub struct PangolinClient {
    /// Subxt client
    subxt: Client<PangolinSubxtConfig>,
}

impl PangolinClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinSubxtConfig>) -> Self {
        Self { subxt: client }
    }
}

impl PangolinClient {
    /// Get original subxt client
    pub fn subxt(&self) -> &Client<PangolinSubxtConfig> {
        &self.subxt
    }
}

/// patch rpc api
impl PangolinClient {
    /// From ethereum api
    pub fn eth_from(&self) -> FromEthereumApi {
        FromEthereumApi::new(&self)
    }

    /// To ethereum api
    pub fn eth_to(&self) -> ToEthereumApi {
        ToEthereumApi::new(&self)
    }
}
