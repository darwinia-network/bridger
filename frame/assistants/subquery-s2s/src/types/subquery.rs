use serde::{Deserialize, Serialize};

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
    pub(crate) block: u32,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
