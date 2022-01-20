use crate::client::PangolinClient;

/// Ethereum api
pub struct EthereumApi<'a> {
    client: &'a PangolinClient,
}

impl<'a> EthereumApi<'a> {
    pub fn new(client: &'a PangolinClient) -> Self {
        Self { client }
    }
}

impl<'a> EthereumApi<'a> {}
