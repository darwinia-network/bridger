use super::types::{
    BlockMessage, Finality, ForkVersion, GetBlockResponse, GetHeaderResponse, Proof,
    ResponseWrapper, Snapshot,
};
use support_common::error::BridgerError;

pub struct KilnClient {
    api_client: reqwest::Client,
    api_base_url: String,
}

impl KilnClient {
    pub fn new(api_endpoint: &str) -> color_eyre::Result<Self> {
        // let transport = Http::new(endpoint)?;
        // let client = web3::Web3::new(transport);
        let api_client = reqwest::Client::new();
        return Ok(Self {
            // client,
            api_client,
            api_base_url: String::from(api_endpoint),
        });
    }

    pub async fn get_header(&self, id: impl ToString) -> color_eyre::Result<GetHeaderResponse> {
        let url = format!(
            "{}/eth/v1/beacon/headers/{}",
            self.api_base_url,
            id.to_string()
        );
        let res: ResponseWrapper<GetHeaderResponse> =
            self.api_client.get(url).send().await?.json().await?;
        Ok(res.data)
    }

    pub async fn find_valid_header_since(
        &self,
        current_slot: u64,
        mut slot: u64,
    ) -> color_eyre::Result<(u64, GetHeaderResponse)> {
        let mut header = self.get_header(slot).await;
        let mut count = 0;
        while let Err(err) = header {
            if slot > current_slot {
                tracing::info!(
                    target: "pangoro-kiln",
                    "[header-kiln-to-pangoro] Wait for attested headers since: {:?}",
                    slot - count
                );
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
    ) -> color_eyre::Result<Option<(u64, u64, GetHeaderResponse, BlockMessage)>> {
        loop {
            if slot > current_slot {
                return Ok(None);
            }
            match self.find_valid_header_since(current_slot, slot).await {
                Ok((attest_slot, header)) => {
                    let (sync_slot, sync_header) = self
                        .find_valid_header_since(current_slot, attest_slot + 1)
                        .await?;
                    let sync_block = self.get_beacon_block(sync_slot).await?;
                    match Self::is_valid_sync_aggregate_block(&sync_block)? {
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

    fn is_valid_sync_aggregate_block(block: &BlockMessage) -> color_eyre::Result<bool> {
        let bytes = hex::decode(&block.body.sync_aggregate.sync_committee_bits.clone()[2..]);
        if let Ok(bytes) = bytes {
            Ok(hamming::weight(&bytes) * 3 > 512 * 2)
        } else {
            Err(BridgerError::Custom(String::from("Failed to decode sync_committee_bits")).into())
        }
    }

    pub async fn get_beacon_block_root(&self, id: impl ToString) -> color_eyre::Result<String> {
        let url = format!(
            "{}/eth/v1/beacon/blocks/{}/root",
            self.api_base_url,
            id.to_string()
        );
        let res: ResponseWrapper<String> = self.api_client.get(url).send().await?.json().await?;
        Ok(res.data)
    }

    pub async fn get_light_client_snapshot(
        &self,
        block_root: &str,
    ) -> color_eyre::Result<Snapshot> {
        let url = format!(
            "{}/eth/v1/lightclient/snapshot/{}",
            self.api_base_url, block_root,
        );
        let res: ResponseWrapper<Snapshot> = self.api_client.get(url).send().await?.json().await?;
        Ok(res.data)
    }

    pub async fn get_beacon_block(&self, id: impl ToString) -> color_eyre::Result<BlockMessage> {
        let url = format!(
            "{}/eth/v2/beacon/blocks/{}",
            self.api_base_url,
            id.to_string(),
        );
        let res: ResponseWrapper<GetBlockResponse> =
            self.api_client.get(url).send().await?.json().await?;
        Ok(res.data.message)
    }

    pub async fn get_checkpoint(&self, id: impl ToString) -> color_eyre::Result<Finality> {
        let url = format!(
            "{}/eth/v1/beacon/states/{}/finality_checkpoints",
            self.api_base_url,
            id.to_string(),
        );
        let res: ResponseWrapper<Finality> = self.api_client.get(url).send().await?.json().await?;
        Ok(res.data)
    }

    pub async fn get_finality_branch(&self, state_id: impl ToString) -> color_eyre::Result<Proof> {
        self.get_state_proof(state_id, 105).await
    }

    pub async fn get_next_sync_committee_branch(
        &self,
        state_id: impl ToString,
    ) -> color_eyre::Result<Proof> {
        self.get_state_proof(state_id, 55).await
    }

    pub async fn get_latest_execution_payload_state_root_branch(
        &self,
        state_id: impl ToString,
    ) -> color_eyre::Result<Proof> {
        self.get_state_proof(state_id, 898).await
    }

    pub async fn get_state_proof(
        &self,
        state_id: impl ToString,
        gindex: impl ToString,
    ) -> color_eyre::Result<Proof> {
        let url = format!(
            "{}/eth/v1/lightclient/single_proof/{}?gindex={}",
            self.api_base_url,
            state_id.to_string(),
            gindex.to_string(),
        );
        let res = self
            .api_client
            .get(url)
            .header("content-type", "application/octet-stream")
            .send()
            .await?
            .bytes()
            .await?;
        Ok(Proof::from(res))
    }

    pub async fn get_fork_version(&self, id: impl ToString) -> color_eyre::Result<ForkVersion> {
        let url = format!(
            "{}/eth/v1/beacon/states/{}/fork",
            self.api_base_url,
            id.to_string(),
        );
        let res: ResponseWrapper<ForkVersion> =
            self.api_client.get(url).send().await?.json().await?;
        Ok(res.data)
    }

    pub async fn get_sync_committee_period_update() -> color_eyre::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_client() -> KilnClient {
        KilnClient::new("http://localhost:5052").unwrap()
    }

    #[tokio::test]
    async fn test_get_header() {
        let client = test_client();
        let header = client.get_header(651232).await.unwrap();
        println!("Header at slot 651232: {:?}", header);

        let header = client.get_header("finalized").await.unwrap();
        println!("Finalized header: {:?}", header);
    }

    #[tokio::test]
    async fn test_get_beacon_block_root() {
        let client = test_client();
        let block_root = client.get_beacon_block_root(651232).await.unwrap();
        println!("Block root at slot 651232: {:?}", block_root);
    }

    #[tokio::test]
    async fn test_get_light_client_snapshot() {
        let client = test_client();
        let snapshot = client
            .get_light_client_snapshot(
                "0xc3873d516be87b55b7729fa4ad06f33ce7b16076ac828e206bfeb85f2b1377e2",
            )
            .await
            .unwrap();
        println!("Block snapshot: {:?}", snapshot);
    }

    #[tokio::test]
    async fn test_get_beacon_block() {
        let client = test_client();
        let block_body = client.get_beacon_block(801823u32).await.unwrap();
        println!("Block body: {:?}", block_body.body.sync_aggregate);
    }

    #[tokio::test]
    async fn test_get_checkpoint() {
        let client = test_client();
        let checkpoint = client.get_checkpoint(801823u32).await.unwrap();
        println!("Checkpoint: {:?}", checkpoint);
    }

    #[tokio::test]
    async fn test_get_state_proof() {
        let client = test_client();
        let proof = client.get_state_proof(801823u32, 55u32).await.unwrap();
        println!("Single proof: {:?}", proof);
    }

    #[tokio::test]
    async fn test_get_fork_version() {
        let client = test_client();
        let fork_version = client.get_fork_version(801823u32).await.unwrap();
        println!("Fork version: {:?}", fork_version);
    }
}
