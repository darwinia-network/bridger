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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
