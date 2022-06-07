use serde::{Deserialize, Serialize};

use crate::types::RelayBlockOrigin;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryNextRelayBlockVars {
    pub(crate) block: u32,
}

#[derive(Clone, Debug, Serialize)]
pub enum OriginType {
    #[serde(rename = "mandatory")]
    Mandatory,
    #[serde(rename = "bridge-pangoro")]
    BridgePangoro,
    #[serde(rename = "bridge-pangolin-parachain")]
    BridgePangolinParachain,
    #[serde(rename = "bridge-pangolin")]
    BridgePangolin,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryNextOnDemandBlockVars {
    pub(crate) origin: OriginType,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct FindJustificationVars {
    pub(crate) hash: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryBlockVars {
    pub(crate) block_number: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryNeedRelay {
    pub origin: RelayBlockOrigin,
    pub lane: String,
    pub nonce: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
