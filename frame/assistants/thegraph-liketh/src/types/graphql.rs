use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) from: u64,
    pub(crate) first: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryMessageEventVars {
    pub(crate) nonce: u64,
}
