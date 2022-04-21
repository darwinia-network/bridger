use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryNextRelayBlockVars {
    pub(crate) block: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct FindJustificationVars {
    pub(crate) hash: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryNextCandidateIncludedEventVars {
    pub(crate) para_id: u32,
    pub(crate) block_number: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
