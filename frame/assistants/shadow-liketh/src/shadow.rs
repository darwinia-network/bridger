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
    //
    pub async fn mmr_root(&self, leaf_index: u64) -> ShadowComponentReuslt<[u8; 32]> {
        let position = mmr_client::mmr::leaf_index_to_pos(leaf_index);
        let peak_positions = mmr_client::mmr::get_peaks(position);
        let query = self.read_graphql("mmr_position.query.graphql")?;
        let vars = QueryPositionVars {
            positions: peak_positions,
        };
        let response = self
            .gql
            .query_with_vars_unwrap::<TheGraphResponse, QueryPositionVars>(query, vars)
            .await
            .map_err(ShadowComponentError::from)?;

        if let TheGraphResponse::NodeEntities(data) = response {
            let peaks = data
                .iter()
                .map(|item| item.hash.clone())
                .map(|item| {
                    let array: arrayvec::ArrayVec<_, 32> = item.into_iter().collect();
                    array
                        .into_inner()
                        .map_err(|e| ShadowComponentError::MMR(format!("{:?}", e)))
                })
                .collect::<Vec<ShadowComponentReuslt<[u8; 32]>>>()
                .into_iter()
                .collect::<ShadowComponentReuslt<Vec<[u8; 32]>>>()?;

            let mmr_root = mmr_client::mmr::bag_rhs_peaks(peaks)
                .map_err(|e| ShadowComponentError::MMR(format!("{:?}", e)))?;
            return Ok(mmr_root);
        }
        Err(ShadowComponentError::GraphQL(format!(
            "Unknown response: {}",
            query
        )))
    }

    // pub asycn fn mmr_proof()
}
