use crate::codegen::api::RuntimeApi;
use subxt::Client;

use crate::config::PangolinSubxtConfig;
use crate::ethereum::{EthereumApi, FromEthereumApi, ToEthereumApi};

/// Pangolin client
#[derive(Clone)]
pub struct PangolinClient {
    /// Subxt client
    subxt: Client<PangolinSubxtConfig>,
    runtime: RuntimeApi<PangolinSubxtConfig>,
}

impl PangolinClient {
    /// Create a new pangolin client
    pub fn new(client: Client<PangolinSubxtConfig>) -> Self {
        let runtime = RuntimeApi::from(client.clone());
        Self {
            subxt: client,
            runtime,
        }
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
    /// Runtime api
    pub fn runtime(&self) -> &RuntimeApi<PangolinSubxtConfig> {
        &self.runtime
    }

    /// Ethereum api
    pub fn ethereum(&self) -> EthereumApi {
        EthereumApi::new(&self)
    }
}
