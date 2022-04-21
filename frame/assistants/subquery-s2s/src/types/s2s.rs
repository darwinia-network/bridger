use serde::{Deserialize, Serialize};
use serde_hex::SerHexSeq;
use serde_hex::StrictPfx;

/// need relay block
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NeedRelayBlock {
    /// id
    pub id: String,
    /// block number
    #[serde(rename = "blockNumber")]
    pub block_number: u32,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "type")]
    pub type_: RelayBlockType,
    pub origin: RelayBlockOrigin,
    #[serde(rename = "laneId")]
    pub lane_id: Option<String>,
    #[serde(rename = "messageNonce")]
    pub message_nonce: Option<u64>,
}

impl NeedRelayBlock {
    /// is mandatory block
    pub fn is_mandatory(&self) -> bool {
        self.type_ == RelayBlockType::Mandatory
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum RelayBlockType {
    Mandatory,
    OnDemand,
}

#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum RelayBlockOrigin {
    Mandatory,
    BridgePanglin,           // from pangolin parachain send message to pangolin
    BridgePangoro,           // from pangolin send message to pangoro
    BridgePangolinParachain, // from pangolin send message to pangolin parachain
}

/// justification mapping
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JustificationMapping {
    /// id
    pub id: String,
    #[serde(rename = "blockNumber")]
    pub block_number: u32,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    pub mandatory: bool,
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    pub justification: Vec<u8>,
}
