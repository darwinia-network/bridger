use subxt::Client;

use crate::client::PangolinClient;
use crate::config::PangolinSubxtConfig;

/// From ethereum api
#[derive(Clone)]
pub struct ToEthereumApi<'a> {
    /// Pangolin client
    client: &'a PangolinClient,
}

impl<'a> ToEthereumApi<'a> {
    /// Create new to ethereum api
    pub fn new(client: &'a PangolinClient) -> Self {
        Self { client }
    }
}
