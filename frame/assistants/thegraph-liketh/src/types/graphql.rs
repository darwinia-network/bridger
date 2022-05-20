use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) from: u64,
    pub(crate) first: u32,
}
