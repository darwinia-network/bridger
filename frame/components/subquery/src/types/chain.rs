use serde::{Deserialize, Serialize};
use serde_hex::SerHexSeq;
use serde_hex::StrictPfx;

use crate::types::DataWrapper;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MMRRootSignedEvent {
    #[serde(rename = "atBlockNumber")]
    pub at_block_number: u32,
    #[serde(rename = "eventBlockNumber")]
    pub event_block_number: u32,
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    #[serde(rename = "mmrRoot")]
    pub mmr_root: Vec<u8>,
    pub signatures: DataWrapper<Signature>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub account: String,
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    #[serde(rename = "relayAuthoritySignature")]
    pub relay_authority_signature: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleMMRRootEvent {
    #[serde(rename = "atBlockNumber")]
    pub at_block_number: u32,
    #[serde(rename = "eventBlockNumber")]
    pub event_block_number: u32,
    pub emitted: u32,
    pub outdated: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleAuthoritiesChangeEvent {
    #[serde(rename = "atBlockNumber")]
    pub at_block_number: u32,
    #[serde(with = "SerHexSeq::<StrictPfx>")]
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthoritiesChangeSignedEvent {
    #[serde(rename = "atBlockNumber")]
    pub at_block_number: u32,
    pub term: u32,
    #[serde(rename = "newAuthorities")]
    #[serde(deserialize_with = "crate::types::patch::smart_vec_string_to_vec_hex")]
    pub new_authorities: Vec<Vec<u8>>,
    pub signatures: DataWrapper<Signature>,
}
