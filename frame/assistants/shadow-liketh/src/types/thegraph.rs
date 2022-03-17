use crate::types::MMRPosition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryPositionVars {
    pub(crate) positions: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[serde(rename = "nodeEntities")]
    NodeEntities(Vec<MMRPosition>),
}
