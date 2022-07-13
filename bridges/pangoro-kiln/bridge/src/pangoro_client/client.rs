use secp256k1::SecretKey;
use std::str::FromStr;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256},
    Web3,
};

use crate::pangoro_client::types::BeaconBlockHeader;

pub struct PangoroClient {
    pub client: Web3<Http>,
    pub contract: Contract<Http>,
    pub execution_layer_contract: Contract<Http>,
    pub private_key: Option<SecretKey>,
}

impl PangoroClient {
    pub fn new(
        endpoint: &str,
        contract_address: &str,
        execution_layer_contract_address: &str,
        private_key: Option<&str>,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = web3::Web3::new(transport);
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(contract_address)?,
            include_bytes!("BeaconLightClient.json"),
        )?;
        let execution_layer_contract = Contract::from_json(
            client.eth(),
            Address::from_str(execution_layer_contract_address)?,
            include_bytes!("ExecutionLayer.json"),
        )?;
        let private_key = private_key.map(SecretKey::from_str).transpose()?;
        Ok(Self {
            client,
            contract,
            execution_layer_contract,
            private_key,
        })
    }

    pub async fn finalized_header(&self) -> color_eyre::Result<BeaconBlockHeader> {
        let query = self
            .contract
            .query("finalized_header", (), None, Options::default(), None);
        let header: BeaconBlockHeader = query.await?;
        Ok(header)
    }

    pub async fn sync_committee_roots(&self, period: u64) -> color_eyre::Result<H256> {
        let query = self.contract.query(
            "sync_committee_roots",
            (period,),
            None,
            Options::default(),
            None,
        );
        let root: H256 = query.await?;
        Ok(root)
    }

    pub async fn execution_layer_state_root(&self) -> color_eyre::Result<H256> {
        let query =
            self.execution_layer_contract
                .query("state_root", (), None, Options::default(), None);
        Ok(query.await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> PangoroClient {
        PangoroClient::new(
            "https://pangoro-rpc.darwinia.network",
            "0x9920317f841F3653464bf37512c939744502CA74",
            "0x99B9C72c93EBC472Ce1A27e064067E78FDcb36E9",
            None,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_query_finalized_header() {
        let client = test_client();
        let header = client.finalized_header().await.unwrap();
        println!("Finalized header: {:?}", header);
    }

    #[tokio::test]
    async fn test_query_sync_committee_roots() {
        let client = test_client();
        let sync_committee_root = client.sync_committee_roots(12).await.unwrap();
        println!("Sync committee root: {:?}", sync_committee_root);
    }

    #[tokio::test]
    async fn test_query_execution_layer_state_root() {
        let client = test_client();
        let state_root = client.execution_layer_state_root().await.unwrap();
        println!("Execution layer state root: {:?}", state_root);
    }
}
