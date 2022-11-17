pub use self::query_vars::*;
#[cfg(feature = "bridge-parachain")]
pub use self::schema_relaychain::*;
pub use self::schema_types::*;

// query variable types
mod query_vars {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize)]
    pub(crate) struct QueryNextRelayBlockVars {
        pub(crate) block: u32,
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
        pub origin: OriginType,
        pub lane: String,
        pub nonce: u64,
    }

    #[cfg(feature = "bridge-parachain")]
    #[derive(Clone, Debug, Serialize)]
    pub(crate) struct QueryNextCandidateIncludedEventWithParaHeadVars {
        pub(crate) para_head: String,
    }

    #[cfg(feature = "bridge-parachain")]
    #[derive(Clone, Debug, Serialize)]
    pub(crate) struct QueryNextCandidateIncludedEventVars {
        pub(crate) para_id: u32,
        pub(crate) block_number: u32,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum OriginType {
        #[serde(rename = "mandatory")]
        Mandatory,
        #[serde(rename = "bridge-pangoro")]
        BridgePangoro,
        #[serde(rename = "bridge-pangolin-parachain")]
        BridgePangolinParachain,
        #[serde(rename = "bridge-pangolin-parachainalpha")]
        BridgePangolinParachainAlpha,
        #[serde(rename = "bridge-pangolin")]
        BridgePangolin,
        #[serde(rename = "bridge-crab-parachain")]
        BridgeCrabParachain,
        #[serde(rename = "bridge-crab")]
        BridgeCrab,
        #[serde(rename = "bridge-darwinia")]
        BridgeDarwinia,
        #[serde(rename = "bridge-darwinia-parachain")]
        BridgeDarwiniaParachain,
    }
}

// schema types
mod schema_types {
    use serde::{Deserialize, Serialize};
    use serde_hex::{SerHex, SerHexSeq, StrictPfx};

    use crate::SubqueryComponentResult;

    use super::OriginType;

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
        pub origin: OriginType,
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
            matches!(self.type_, RelayBlockType::Mandatory)
        }

        pub fn block_hash_bytes(&self) -> SubqueryComponentResult<Vec<u8>> {
            Ok(array_bytes::hex2bytes(&self.block_hash)?)
        }
    }

    #[derive(
        Clone,
        Debug,
        Deserialize,
        Serialize,
        Eq,
        PartialEq,
        strum::EnumString,
        strum::EnumVariantNames,
    )]
    #[strum(serialize_all = "kebab_case")]
    #[serde(rename_all = "kebab-case")]
    pub enum RelayBlockType {
        Mandatory,
        OnDemand,
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
}

#[cfg(feature = "bridge-parachain")]
mod schema_relaychain {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CandidateIncludedEvent {
        pub id: String,

        /// block number of relay chain
        #[serde(rename = "includedRelayBlock")]
        pub included_relay_block: u32,

        #[serde(rename = "paraId")]
        pub para_id: u32,

        pub signature: String,

        /// parachain block hash
        #[serde(rename = "paraHead")]
        pub para_head: String,

        #[serde(rename = "relayParent")]
        pub relay_parent: String,
    }
}
