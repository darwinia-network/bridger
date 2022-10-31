use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryMessageEventVars {
    pub(crate) nonce: u64,
}
