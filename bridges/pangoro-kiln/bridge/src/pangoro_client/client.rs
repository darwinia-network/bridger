use secp256k1::SecretKey;
use std::{fs, str::FromStr};
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
    pub private_key: SecretKey,
}

impl PangoroClient {
    pub fn new(
        endpoint: &str,
        abi_path: &str,
        contract_address: &str,
        private_key: &str,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = web3::Web3::new(transport);
        let abi = fs::read(abi_path)?;
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(contract_address)?,
            abi.as_slice(),
        )?;
        let private_key = SecretKey::from_str(private_key)?;
        Ok(Self {
            client,
            contract,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> PangoroClient {
        PangoroClient::new(
            "https://pangoro-rpc.darwinia.network",
         "/Users/furoxr/Projects/bridger/frame/abstract/bridge-s2e/src/BeaconLightClient_abi.json",
         "0xedD0683d354b2d2c209Ac8c574ef88E85bdBEa70",
         "//Alice"
            ).unwrap()
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
        let sync_committee_root = client.sync_committee_roots(79).await.unwrap();
        println!("Sync committee root: {:?}", sync_committee_root);
    }
}
