use super::types::{
    Finality, FinalityUpdate, ForkVersion, GetHeaderResponse, Proof, ResponseWrapper, Snapshot,
    SyncCommitteePeriodUpdate,
};
use crate::{
    error::{BeaconApiError, BeaconApiResult},
    types::{BeaconBlockRoot, BeaconBlockWrapper, ErrorResponse},
};
use reqwest::{header::CONTENT_TYPE, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use types::{MainnetEthSpec, BeaconBlock};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ApiSupplier {
    Lodestar,
    Nimbus,
}

pub struct BeaconApiClient {
    api_client: reqwest::Client,
    api_base_url: String,
    api_supplier: ApiSupplier,
}

impl BeaconApiClient {
    pub fn new(api_endpoint: &str, api_supplier: ApiSupplier) -> BeaconApiResult<Self> {
        let api_client = reqwest::Client::new();
        Ok(Self {
            api_client,
            api_base_url: String::from(api_endpoint),
            api_supplier,
        })
    }

    fn get(&self, url: &str) -> RequestBuilder {
        tracing::trace!(target: "client-beacon", "Request to {:?}", &url);
        self.api_client.get(url)
    }

    async fn parse_reponse<R: DeserializeOwned>(response: Response) -> BeaconApiResult<R> {
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let url: String = response.url().as_str().into();
            let status_code = response.status();
            let res: ErrorResponse = response.json().await.map_err(|_| {
                BeaconApiError::Custom(format!(
                    "Failed to connect to beacon api servcice. url: {:?}, status code: {:?}",
                    url, status_code
                ))
            })?;
            Err(BeaconApiError::BeaconApiError {
                status_code: res.status_code,
                error: res.error,
                message: res.message,
            })
        }
    }

    pub async fn get_header(&self, id: impl ToString) -> BeaconApiResult<GetHeaderResponse> {
        let url = format!(
            "{}/eth/v1/beacon/headers/{}",
            self.api_base_url,
            id.to_string()
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<GetHeaderResponse> = Self::parse_reponse(response).await?;
        Ok(res.data)
    }

    pub async fn find_valid_header_since(
        &self,
        current_slot: u64,
        mut slot: u64,
    ) -> BeaconApiResult<(u64, GetHeaderResponse)> {
        let mut header = self.get_header(slot).await;
        let mut count = 0;
        while let Err(err) = header {
            if slot > current_slot {
                tracing::info!("Wait for attested headers since: {:?}", slot - count);
                return Err(err);
            };
            slot += 1;
            header = self.get_header(slot).await;
            count += 1;
        }
        Ok((slot, header.expect("Unreachable")))
    }

    pub async fn find_valid_attested_header(
        &self,
        current_slot: u64,
        mut slot: u64,
    ) -> BeaconApiResult<
        Option<(
            u64,
            u64,
            GetHeaderResponse,
            BeaconBlock<MainnetEthSpec>,
        )>,
    > {
        loop {
            if slot > current_slot {
                return Ok(None);
            }
            match self.find_valid_header_since(current_slot, slot).await {
                Ok((attest_slot, header)) => {
                    let (sync_slot, _sync_header) = self
                        .find_valid_header_since(current_slot, attest_slot + 1)
                        .await?;

                    let sync_block = self.get_beacon_block(sync_slot).await?;
                    match Self::is_valid_sync_aggregate_block(&sync_block.body().sync_aggregate().unwrap().sync_committee_bits.as_slice())? {
                        true => return Ok(Some((attest_slot, sync_slot, header, sync_block))),
                        false => {
                            slot += 1;
                            continue;
                        }
                    }
                }
                Err(_) => {
                    slot += 1;
                    continue;
                }
            }
        }
    }

    fn is_valid_sync_aggregate_block(
        sync_committee_bits: &[u8],
    ) -> BeaconApiResult<bool> {
        Ok(hamming::weight(sync_committee_bits) * 3 > 512 * 2)
    }

    pub async fn get_beacon_block_root(&self, id: impl ToString) -> BeaconApiResult<String> {
        let url = format!(
            "{}/eth/v1/beacon/blocks/{}/root",
            self.api_base_url,
            id.to_string()
        );
        let response = self.get(&url).send().await?;
        let res = Self::parse_reponse::<ResponseWrapper<BeaconBlockRoot>>(response).await?;
        Ok(res.data.root)
    }

    pub async fn get_bootstrap(&self, header_root: &str) -> BeaconApiResult<Snapshot> {
        let url = format!(
            "{}/eth/v1/beacon/light_client/bootstrap/{}",
            self.api_base_url, header_root,
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<Snapshot> = Self::parse_reponse(response).await?;
        Ok(res.data)
    }

    #[allow(dead_code)]
    pub async fn find_valid_snapshot_in_period(&self, period: u64) -> BeaconApiResult<Snapshot> {
        let begin_slot = period * 32 * 256;
        for slot in begin_slot..((period + 1) * 32 * 256) {
            if let Ok(block_root) = self.get_beacon_block_root(slot).await {
                if let Ok(snapshot) = self.get_bootstrap(&block_root).await {
                    return Ok(snapshot);
                }
            };
        }
        Err(BeaconApiError::Custom("Not found valid snapshot".into()).into())
    }

    pub async fn get_beacon_block(
        &self,
        id: impl ToString,
    ) -> BeaconApiResult<BeaconBlock<MainnetEthSpec>> {
        let url = format!(
            "{}/eth/v2/beacon/blocks/{}",
            self.api_base_url,
            id.to_string(),
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<BeaconBlockWrapper> = Self::parse_reponse(response).await?;
        Ok(res.data.message)
    }

    #[allow(dead_code)]
    pub async fn get_checkpoint(&self, id: impl ToString) -> BeaconApiResult<Finality> {
        let url = format!(
            "{}/eth/v1/beacon/states/{}/finality_checkpoints",
            self.api_base_url,
            id.to_string(),
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<Finality> = Self::parse_reponse(response).await?;
        Ok(res.data)
    }

    #[allow(dead_code)]
    pub async fn get_finality_branch(&self, state_id: impl ToString) -> BeaconApiResult<Proof> {
        self.get_state_proof(state_id, 105).await
    }

    pub async fn get_next_sync_committee_branch(
        &self,
        state_id: impl ToString,
    ) -> BeaconApiResult<Proof> {
        self.get_state_proof(state_id, 55).await
    }

    pub async fn get_latest_execution_payload_state_root_branch(
        &self,
        state_id: impl ToString,
    ) -> BeaconApiResult<Proof> {
        self.get_state_proof(state_id, 898).await
    }

    pub async fn get_state_proof(
        &self,
        state_id: impl ToString,
        gindex: impl ToString,
    ) -> BeaconApiResult<Proof> {
        let url = format!(
            "{}/eth/v1/beacon/light_client/single_proof/{}?gindex={}",
            self.api_base_url,
            state_id.to_string(),
            gindex.to_string(),
        );
        let response = self
            .api_client
            .get(url.clone())
            .header("content-type", "application/octet-stream")
            .send()
            .await?;
        let content_type = response.headers()[CONTENT_TYPE].as_bytes().to_vec();
        let content_type = String::from_utf8(content_type)?;
        if !response.status().is_success() || content_type.contains("application/json") {
            tracing::error!("Failed to get state proof. Api: {:?}", url);
            return Err(BeaconApiError::Custom("Failed to get state proof".into()));
        }

        let data = response.bytes().await?;
        Ok(Proof::try_from(data)?)
    }

    pub async fn get_fork_version(&self, id: impl ToString) -> BeaconApiResult<ForkVersion> {
        let url = format!(
            "{}/eth/v1/beacon/states/{}/fork",
            self.api_base_url,
            id.to_string(),
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<ForkVersion> = Self::parse_reponse(response).await?;
        Ok(res.data)
    }

    pub async fn get_finality_update(&self) -> BeaconApiResult<FinalityUpdate> {
        let url = format!(
            "{}/eth/v1/beacon/light_client/finality_update",
            self.api_base_url,
        );
        let response = self.get(&url).send().await?;
        let res: ResponseWrapper<FinalityUpdate> = Self::parse_reponse(response).await?;
        Ok(res.data)
    }

    pub async fn get_sync_committee_period_update(
        &self,
        start_period: impl ToString,
        count: impl ToString,
    ) -> BeaconApiResult<Vec<SyncCommitteePeriodUpdate>> {
        let url = format!(
            "{}/eth/v1/beacon/light_client/updates?start_period={}&count={}",
            self.api_base_url,
            start_period.to_string(),
            count.to_string(),
        );
        let response = self.get(&url).send().await?;
        let result = match self.api_supplier {
            ApiSupplier::Nimbus => {
                Self::parse_reponse::<Vec<ResponseWrapper<SyncCommitteePeriodUpdate>>>(response)
                    .await?
                    .into_iter()
                    .map(|x| x.data)
                    .collect()
            }
            ApiSupplier::Lodestar => {
                Self::parse_reponse::<ResponseWrapper<Vec<SyncCommitteePeriodUpdate>>>(response)
                    .await?
                    .data
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use tree_hash::TreeHash;
    use types::ExecPayload;

    use super::*;

    fn test_client() -> BeaconApiClient {
        // BeaconApiClient::new("http://g2.dev.darwinia.network:9596").unwrap()
        BeaconApiClient::new("https://lodestar-goerli.chainsafe.io", ApiSupplier::Nimbus).unwrap()
    }

    // #[ignore]
    #[tokio::test]
    async fn test_get_header() {
        let client = test_client();
        let slot = 4382849;
        let header = client.get_header(slot).await.unwrap();
        println!("Header at slot {}  : {:?}", slot, header);

        let header = client.get_header("finalized").await.unwrap();
        println!("Finalized header: {:?}", header);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_beacon_block_root() {
        let client = test_client();
        let block_root = client.get_beacon_block_root(120960).await.unwrap();
        println!("Block root at slot 120960: {:?}", block_root);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_bootstrap() {
        let client = test_client();

        let header_root = client.get_beacon_block_root(120960).await.unwrap();
        println!("Header root: {:?}", header_root);
        let snapshot = client.get_bootstrap(&header_root).await.unwrap();
        println!("Block snapshot: {:?}", snapshot);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_beacon_block() {
        let client = test_client();
        let block_body = client.get_beacon_block(5202400).await.unwrap();
        println!(
            "Block body: {:?}",
            block_body.body().execution_payload().unwrap().block_hash()
        );
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_checkpoint() {
        let client = test_client();
        let checkpoint = client.get_checkpoint(120960).await.unwrap();
        println!("Checkpoint: {:?}", checkpoint);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_state_proof() {
        let client = test_client();
        let proof = client.get_state_proof(120960u32, 55u32).await.unwrap();
        println!("Single proof: {:?}", proof);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_fork_version() {
        let client = test_client();
        let fork_version = client.get_fork_version(120960).await.unwrap();
        println!("Fork version: {:?}", fork_version);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_sync_committee_period_update() {
        let client = test_client();
        let update = client
            .get_sync_committee_period_update(12, 1)
            .await
            .unwrap();
        println!("Update: {:?}", update);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_finality_update() {
        let client = test_client();
        let finality_update = client.get_finality_update().await.unwrap();
        println!("Finality update: {:?}", finality_update);
    }
}
