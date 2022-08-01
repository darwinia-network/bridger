use crate::{
    kiln_client::types::{HeaderMessage, SyncAggregate, SyncCommittee},
    pangoro_client::types::BeaconBlockHeader,
};
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

    #[allow(clippy::too_many_arguments)]
    pub async fn import_finalized_header(
        &self,
        attested_header: &HeaderMessage,
        signature_sync_committee: &SyncCommittee,
        finalized_header: &HeaderMessage,
        finality_branch: &[String],
        sync_aggregate: &SyncAggregate,
        fork_version: &H32,
        signature_slot: u64,
    ) -> color_eyre::Result<H256> {
        let parameter = Token::Tuple(
            (
                attested_header.get_token()?,
                signature_sync_committee.get_token()?,
                finalized_header.get_token()?,
                finality_branch
                    .iter()
                    .map(|x| H256::from_str(x))
                    .collect::<Result<Vec<H256>, _>>()?,
                sync_aggregate.get_token()?,
                Token::FixedBytes(fork_version.as_bytes().to_vec()),
                signature_slot,
            )
                .into_tokens(),
        );
        let tx = self
            .contract
            .signed_call(
                "import_finalized_header",
                (parameter,),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    ..Default::default()
                },
                &self.private_key.ok_or_else(|| {
                    BridgerError::Custom("Failed to get log_bloom from block".into())
                })?,
            )
            .await?;
        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] Sending tx: {:?}",
            &tx
        );
        Ok(tx)
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
