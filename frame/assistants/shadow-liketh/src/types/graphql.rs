use serde::{Deserialize, Serialize};

use crate::types::MMRNode;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryPositionVars {
    pub(crate) positions: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[serde(rename = "nodeEntities")]
    NodeEntities(Vec<MMRNode>),
}
