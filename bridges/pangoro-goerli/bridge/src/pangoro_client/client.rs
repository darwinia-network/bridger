use crate::{
    goerli_client::types::{HeaderMessage, SyncAggregate, SyncCommittee},
    pangoro_client::types::BeaconBlockHeader,
};
use client_contracts::beacon_light_client::BeaconLightClient;
use secp256k1::SecretKey;
use std::str::FromStr;
use support_common::error::BridgerError;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    ethabi::{ethereum_types::H32, Token},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

pub struct PangoroClient {
    pub client: Web3<Http>,
    pub beacon_light_client: BeaconLightClient,
    pub execution_layer_contract: Contract<Http>,
    pub private_key: SecretKey,
}

impl PangoroClient {
    pub fn new(
        endpoint: &str,
        contract_address: &str,
        execution_layer_contract_address: &str,
        private_key: &str,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = web3::Web3::new(transport);
        let beacon_light_client =
            BeaconLightClient::new(&client, Address::from_str(contract_address)?)?;
        let execution_layer_contract = Contract::from_json(
            client.eth(),
            Address::from_str(execution_layer_contract_address)?,
            include_bytes!("ExecutionLayer.json"),
        )?;
        let private_key = SecretKey::from_str(private_key)?;
        Ok(Self {
            client,
            beacon_light_client,
            execution_layer_contract,
            private_key,
        })
    }

    pub async fn execution_layer_state_root(&self) -> color_eyre::Result<H256> {
        let query =
            self.execution_layer_contract
                .query("merkle_root", (), None, Options::default(), None);
        Ok(query.await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> PangoroClient {
        PangoroClient::new(
            "https://pangoro-rpc.darwinia.network",
            "0x59EA974B74ec6A49338438bCc5d0388E294E4E20",
            "0x43258d32E29b2C866d882183758B864471A26b96",
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
