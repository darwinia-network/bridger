use subxt::Client;

use crate::client::PangolinClient;
use crate::config::PangolinSubxtConfig;
use crate::error::ClientResult;

/// From ethereum api
#[derive(Clone)]
pub struct FromEthereumApi<'a> {
    /// Pangolin client
    client: &'a PangolinClient,
}

impl<'a> FromEthereumApi<'a> {
    /// Create new from ethereum api
    pub fn new(client: &'a PangolinClient) -> Self {
        Self { client }
    }
}

impl<'a> FromEthereumApi<'a> {
    /// The account is tech comm member
    pub fn is_tech_comm_member(
        &self,
        block_number: Option<u32>,
        account: &Account,
    ) -> ClientResult<bool> {
        let block_number = block_number.map(|v| v.into());
        let block_hash = self.client.subxt().rpc().block_hash(block_number).await?;

        // self.client.subxt().storage().fetch()

        // let tech_comm_members = self.client.subxt().members(block_hash).await?;
        // Ok(tech_comm_members.contains(account.0.real()))
        Ok(true)
    }
}
