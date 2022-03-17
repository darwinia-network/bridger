use std::collections::HashMap;

use include_dir::{include_dir, Dir};

use crate::error::{ShadowComponentError, ShadowComponentReuslt};
use crate::types::{BridgeName, MMRPosition, QueryPositionVars, TheGraphResponse};

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// Shadow client
pub struct Shadow {
    endpoint: String,
    gql: gql_client::Client,
    bridge: BridgeName,
}

impl Shadow {
    /// Create shadow instance
    pub fn new(endpoint: String, gql: gql_client::Client, bridge: BridgeName) -> Self {
        Self {
            endpoint,
            gql,
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
    fn calc_peaks(
        &self,
        positions: Vec<MMRPosition>,
    ) -> ShadowComponentReuslt<Vec<(u64, [u8; 32])>> {
        positions
            .iter()
            .map(|item| {
                let array: arrayvec::ArrayVec<_, 32> = item.hash.clone().into_iter().collect();
                match array
                    .into_inner()
                    .map_err(|e| ShadowComponentError::MMR(format!("{:?}", e)))
                {
                    Ok(v) => Ok((item.id, v)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<ShadowComponentReuslt<(u64, [u8; 32])>>>()
            .into_iter()
            .collect::<ShadowComponentReuslt<Vec<(u64, [u8; 32])>>>()
    }

    async fn query_position(&self, positions: Vec<u64>) -> ShadowComponentReuslt<Vec<MMRPosition>> {
        let query = self.read_graphql("mmr_position.query.graphql")?;

        let vars = QueryPositionVars { positions };
        let response = self
            .gql
            .query_with_vars_unwrap::<TheGraphResponse, QueryPositionVars>(query, vars)
            .await
            .map_err(ShadowComponentError::from)?;

        match response {
            TheGraphResponse::NodeEntities(data) => Ok(data),
            _ => Err(ShadowComponentError::GraphQL(format!(
                "Unknown response: {}",
                query
            ))),
        }
    }

    //
    pub async fn mmr_root(&self, leaf_index: u64) -> ShadowComponentReuslt<[u8; 32]> {
        let position = mmr_client::mmr::leaf_index_to_pos(leaf_index);
        let peak_positions = mmr_client::mmr::get_peaks(position);

        let mmr_positions = self.query_position(peak_positions).await?;
        let peaks = self
            .calc_peaks(mmr_positions)?
            .iter()
            .map(|item| item.1)
            .collect::<Vec<[u8; 32]>>();

        let mmr_root = mmr_client::mmr::bag_rhs_peaks(peaks)
            .map_err(|e| ShadowComponentError::MMR(format!("{:?}", e)))?;
        Ok(mmr_root)
    }

    pub async fn mmr_proof(
        &self,
        tx_number: u64,
        last_leaf: u64,
    ) -> ShadowComponentReuslt<Vec<[u8; 32]>> {
        let tx_position = mmr_client::mmr::leaf_index_to_pos(tx_number);
        let leaf_pos = mmr_client::mmr::leaf_index_to_pos(last_leaf);
        // 1. gen positions
        let (merkle_proof_pos, peaks_pos, peak_pos) =
            mmr_client::mmr::gen_proof_positions(tx_position, leaf_pos);

        let merkle_proof_positions = self.query_position(merkle_proof_pos).await?;
        let merkle_proof = self
            .calc_peaks(merkle_proof_positions)?
            .iter()
            .map(|item| item.1)
            .collect::<Vec<[u8; 32]>>();

        let peaks_positions = self.query_position(peaks_pos).await?;
        let peaks = self.calc_peaks(peaks_positions)?;
        let mmr_proof = mmr_client::mmr::gen_proof(merkle_proof, peaks, peak_pos);
        Ok(mmr_proof)
    }
}
