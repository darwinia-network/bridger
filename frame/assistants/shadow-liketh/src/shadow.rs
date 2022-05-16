use include_dir::{include_dir, Dir};

use component_ethereum::ethereum::client::EthereumClient;
use support_mmr::mmr;

use crate::config::ShadowConfig;
use crate::error::{ShadowComponentError, ShadowComponentReuslt};
use crate::types::{
    BridgeName, EthereumReceiptJson, EthereumReceiptWithMMRProof, HeaderParcel, MMRNode,
    MMRProofJson, QueryPositionVars, TheGraphResponse,
};

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// Shadow client
pub struct Shadow {
    /// Shadow config
    config: ShadowConfig,
    /// gql client
    gql: gql_client::Client,
    /// Ethereum RPC
    eth: EthereumClient,
    /// HTTP Client
    http: reqwest::Client,
    /// Bridge name
    bridge: BridgeName,
}

impl Shadow {
    /// Create shadow instance
    pub fn new(
        config: ShadowConfig,
        gql: gql_client::Client,
        eth: EthereumClient,
        http: reqwest::Client,
        bridge: BridgeName,
    ) -> Self {
        Self {
            config,
            gql,
            eth,
            http,
            bridge,
        }
    }
}

impl Shadow {
    fn read_graphql(&self, file: impl AsRef<str>) -> ShadowComponentReuslt<&str> {
        let file = file.as_ref();
        let dir = self.bridge.directory();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| ShadowComponentError::GraphQL("No graphql fround".to_string()))?;
        graph
            .contents_utf8()
            .ok_or_else(|| ShadowComponentError::GraphQL("Failed to read graphql file".to_string()))
    }
}

impl Shadow {
    pub async fn parcel(&self, block_number: u64) -> ShadowComponentReuslt<HeaderParcel> {
        tracing::debug!(target: "shadow", "Shadow parcel fo block: {}", block_number);
        let header = self
            .eth
            .get_header_by_number(block_number)
            .await
            .map_err(|e| ShadowComponentError::Ethereum(format!("{:?}", e)))?;
        let mmr_root = self.mmr_root(block_number - 1).await?;
        Ok(HeaderParcel { mmr_root, header })
    }

    pub async fn receipt(
        &self,
        tx: impl AsRef<str>,
        last: u64,
    ) -> ShadowComponentReuslt<EthereumReceiptWithMMRProof> {
        let resp = self
            .http
            .get(&format!(
                "{}/ethereum/receipt/{}",
                &self.config.endpoint,
                tx.as_ref(),
            ))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
            return Err(ShadowComponentError::InternalServer(resp.text().await?));
        }
        let result: serde_json::Value = resp.json().await?;
        if let Some(err) = result.get("error") {
            tracing::error!(
                target: "shadow",
                "Error response by shadow: {:?}",
                result,
            );
            let msg = err.as_str().ok_or_else(|| {
                ShadowComponentError::Cusom("Failed parse error message".to_string())
            })?;
            return Err(ShadowComponentError::Cusom(msg.to_owned()));
        }
        let receipt: EthereumReceiptJson = serde_json::from_value(result)?;
        let header = &receipt.header;

        let (member_leaf_index, last_leaf_index) = (header.number, last - 1);
        let proof = self.mmr_proof(member_leaf_index, last_leaf_index).await?;
        let mmr_proof = MMRProofJson {
            member_leaf_index,
            last_leaf_index,
            proof,
        };
        Ok(EthereumReceiptWithMMRProof { receipt, mmr_proof })
    }
}

impl Shadow {
    fn extract_peaks(&self, positions: Vec<MMRNode>) -> Vec<(u64, [u8; 32])> {
        positions
            .iter()
            .map(|item| (item.position, item.hash))
            .collect::<Vec<(u64, [u8; 32])>>()
    }

    async fn query_nodes(&self, positions: Vec<u64>) -> ShadowComponentReuslt<Vec<MMRNode>> {
        let query = self.read_graphql("mmr_position.query.graphql")?;

        let vars = QueryPositionVars { positions };
        let response = self
            .gql
            .query_with_vars_unwrap::<TheGraphResponse, QueryPositionVars>(query, vars)
            .await
            .map_err(ShadowComponentError::from)?;

        match response {
            TheGraphResponse::NodeEntities(data) => Ok(data),
        }
    }

    pub async fn mmr_root(&self, leaf_index: u64) -> ShadowComponentReuslt<[u8; 32]> {
        let mmr_size = mmr::leaf_index_to_mmr_size(leaf_index);
        let peak_positions = mmr::get_peaks(mmr_size);
        tracing::debug!(target: "shadow", "mmr root peak positions: {:?}", peak_positions);

        let mmr_nodes = self.query_nodes(peak_positions).await?;
        let peaks = self
            .extract_peaks(mmr_nodes)
            .iter()
            .map(|item| item.1)
            .collect::<Vec<[u8; 32]>>();

        let mmr_root =
            mmr::bag_rhs_peaks(peaks).map_err(|e| ShadowComponentError::MMR(format!("{:?}", e)))?;
        Ok(mmr_root)
    }

    pub async fn mmr_proof(
        &self,
        tx_number: u64,
        last_leaf: u64,
    ) -> ShadowComponentReuslt<Vec<[u8; 32]>> {
        let verified_leaf_position = mmr::leaf_index_to_pos(tx_number);
        let mmr_size = mmr::leaf_index_to_mmr_size(last_leaf);
        tracing::trace!(
            target: "shadow",
            "mmr proof verified_leaf_position: {}, mmr_size: {}",
            verified_leaf_position,
            mmr_size
        );
        // 1. gen positions
        let (merkle_proof_pos, peak_positions, peak_pos_of_leaf_index) =
            mmr::gen_proof_positions(verified_leaf_position, mmr_size);

        let merkle_proof_positions = self.query_nodes(merkle_proof_pos).await?;
        let merkle_proof = self
            .extract_peaks(merkle_proof_positions)
            .iter()
            .map(|item| item.1)
            .collect::<Vec<[u8; 32]>>();

        let peaks_positions = self.query_nodes(peak_positions).await?;
        let peaks = self.extract_peaks(peaks_positions);
        let mmr_proof = mmr::gen_proof(merkle_proof, peaks, peak_pos_of_leaf_index);
        Ok(mmr_proof)
    }
}
