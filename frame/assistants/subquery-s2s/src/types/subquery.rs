use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) from: u64,
    pub(crate) first: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryBlockVars {
    pub(crate) block_number: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
