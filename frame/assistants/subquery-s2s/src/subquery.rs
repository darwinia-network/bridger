use std::collections::HashMap;

use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::SubqueryComponentError;
use crate::types::{
    BridgeName, DataWrapper, FindJustificationVars, JustificationMapping, NeedRelayBlock,
    OriginType, QueryNeedRelay, QueryNextOnDemandBlockVars, QueryNextRelayBlockVars,
    RelayBlockOrigin,
};
use crate::SubqueryComponentResult;

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// Subquery client
#[derive(Clone, Debug)]
pub struct Subquery {
    client: Client,
    bridge: BridgeName,
}

impl Subquery {
    /// Create subquery instance
    pub fn new(client: Client, bridge: BridgeName) -> Self {
        Self { client, bridge }
    }
}

impl Subquery {
    fn read_graphql(&self, file: impl AsRef<str>) -> SubqueryComponentResult<&str> {
        let file = file.as_ref();
        let dir = self.bridge.directory();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| SubqueryComponentError::GraphQL("No graphql fround".to_string()))?;
        graph.contents_utf8().ok_or_else(|| {
            SubqueryComponentError::GraphQL("Failed to read graphql file".to_string())
        })
    }
}

impl Subquery {
    /// Query next mandatory header
    pub async fn next_mandatory_header(
        &self,
        block_number: u32,
    ) -> SubqueryComponentResult<Option<NeedRelayBlock>> {
        let query = self.read_graphql("next_header.query.graphql")?;
        let vars = QueryNextRelayBlockVars {
            block: block_number,
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<NeedRelayBlock>>, QueryNextRelayBlockVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let blocks = data
            .get("needRelayBlocks")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(blocks.get(0).cloned())
    }

    /// Query next needed header (on-demand)
    pub async fn next_needed_header(
        &self,
        origin: OriginType,
    ) -> SubqueryComponentResult<Option<NeedRelayBlock>> {
        let query = self.read_graphql("next_needed_header.query.graphql")?;
        let vars = QueryNextOnDemandBlockVars { origin };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<NeedRelayBlock>>, QueryNextOnDemandBlockVars>(query, vars)
            .await
            .map_err(SubqueryComponentError::from)?;
        let blocks = data
            .get("needRelayBlocks")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(blocks.get(0).cloned())
    }

    /// Find justification
    pub async fn find_justification(
        &self,
        block_hash: impl AsRef<str>,
        is_mandatory: bool,
    ) -> SubqueryComponentResult<Option<JustificationMapping>> {
        let query_by_hash = self.read_graphql("justification_mapping_by_hash.query.graphql")?;
        let query_latest = self.read_graphql("justification_mapping_latest.query.graphql")?;
        let vars = FindJustificationVars {
            hash: block_hash.as_ref().to_string(),
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<JustificationMapping>>, FindJustificationVars>(
                if is_mandatory { query_by_hash } else { query_latest }, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let justifications = data
            .get("justificationMappings")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(justifications.get(0).cloned())
    }

    /// Query relay info by nonce
    pub async fn query_need_relay(
        &self,
        origin: RelayBlockOrigin,
        lane: [u8; 4],
        nonce: u64,
    ) -> SubqueryComponentResult<Option<NeedRelayBlock>> {
        let query = self.read_graphql("query_need_relay.graphql")?;
        let lane_hex = array_bytes::bytes2hex("", &lane);
        let vars = QueryNeedRelay {
            origin,
            lane: lane_hex,
            nonce,
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<NeedRelayBlock>>, QueryNeedRelay>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let blocks = data
            .get("needRelayBlocks")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(blocks.get(0).cloned())
    }
}
