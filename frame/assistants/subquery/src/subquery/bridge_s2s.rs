use std::collections::HashMap;

#[cfg(feature = "bridge-parachain")]
use crate::types::{CandidateIncludedEvent, QueryNextCandidateIncludedEventWithParaHeadVars};
use crate::types::{
    DataWrapper, FindJustificationVars, JustificationMapping, NeedRelayBlock, OriginType,
    QueryNeedRelay, QueryNextOnDemandBlockVars, QueryNextRelayBlockVars, RelayBlockOrigin,
};
use crate::{Subquery, SubqueryComponentError, SubqueryComponentResult};

impl Subquery {
    /// Query next mandatory header
    pub async fn next_mandatory_header(
        &self,
        block_number: u32,
    ) -> SubqueryComponentResult<Option<NeedRelayBlock>> {
        let query = self.read_graphql("bridge_s2s_next_header.query.graphql")?;
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
        let query = self.read_graphql("bridge_s2s_next_needed_header.query.graphql")?;
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
        let query_by_hash =
            self.read_graphql("bridge_s2s_justification_mapping_by_hash.query.graphql")?;
        let query_latest =
            self.read_graphql("bridge_s2s_justification_mapping_latest.query.graphql")?;
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
        let query = self.read_graphql("bridge_s2s_query_need_relay.graphql")?;
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

#[cfg(feature = "bridge-parachain")]
impl Subquery {
    pub async fn get_block_with_para_head(
        &self,
        para_head_hash: impl AsRef<str>,
    ) -> SubqueryComponentResult<Option<CandidateIncludedEvent>> {
        let query = self.read_graphql(
            "bridge_s2s_next_candidate_included_event_with_para_head.query.graphql",
        )?;
        let vars = QueryNextCandidateIncludedEventWithParaHeadVars {
            para_head: String::from(para_head_hash.as_ref()),
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CandidateIncludedEvent>>, QueryNextCandidateIncludedEventWithParaHeadVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let event = data
            .get("candidateIncludedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(event.get(0).cloned())
    }
}
