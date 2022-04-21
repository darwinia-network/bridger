use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, SerHexSeq, StrictPfx};

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
    #[serde(rename = "parentHash")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub parent_hash: [u8; 32],
    #[serde(rename = "stateRoot")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub state_root: [u8; 32],
    #[serde(rename = "extrinsicsRoot")]
    #[serde(with = "SerHex::<StrictPfx>")]
    pub extrinsics_root: [u8; 32],
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    pub digest: Vec<u8>,
}

impl NeedRelayBlock {
    /// is mandatory block
    pub fn is_mandatory(&self) -> bool {
        if let RelayBlockType::Mandatory = self.type_ {
            true
        } else {
            false
        }
    }
}

#[derive(
    Clone, Debug, Deserialize, Serialize, Eq, PartialEq, strum::EnumString, strum::EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum RelayBlockType {
    Mandatory,
    OnDemand,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, Eq, PartialEq, strum::EnumString, strum::EnumVariantNames,
)]
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
