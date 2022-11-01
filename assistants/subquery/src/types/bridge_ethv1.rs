use self::patch::*;
pub(crate) use self::query_vars::*;
pub use self::schema_types::*;

// query variable types
mod query_vars {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub(crate) struct QueryTransactionsVars {
        pub(crate) from: u64,
        pub(crate) first: u32,
    }
}

mod schema_types {
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
        #[serde(default)]
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
        #[serde(deserialize_with = "crate::types::bridge_ethv1::smart_vec_string_to_vec_hex")]
        pub new_authorities: Vec<Vec<u8>>,
        pub signatures: DataWrapper<Signature>,
    }
}

// schema types

// patch

mod patch {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub(super) fn smart_vec_string_to_vec_hex<'de, D>(
        deserializer: D,
    ) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Vec::<String>::deserialize(deserializer)?;
        let mut r = Vec::with_capacity(s.len());
        for v in s {
            match hex::decode(v) {
                Ok(v) => r.push(v),
                Err(e) => {
                    return Err(D::Error::custom(format!(
                        "can't deserialize to Vec<Vec<u8>>: {}",
                        e
                    )));
                }
            }
        }
        Ok(r)
    }
}
