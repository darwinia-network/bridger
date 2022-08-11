pub use self::query_vars::*;
pub use self::schema_types::*;

mod query_vars {
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub(crate) struct QueryWithBlockNumberVars {
        pub(crate) block: u32,
    }
}

mod schema_types {
    use serde::{Deserialize, Serialize};
    use serde_hex::SerHexSeq;
    use serde_hex::StrictPfx;

    use crate::types::DataWrapper;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EcdsaSignature {
        pub id: String,
        #[serde(rename = "blockNumber")]
        pub block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "blockHash")]
        pub block_hash: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub address: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub signature: Vec<u8>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CollectingNewMessageRootSignaturesEvent {
        pub id: String,
        #[serde(rename = "blockNumber")]
        pub block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "blockHash")]
        pub block_hash: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub message: Vec<u8>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CollectingAuthoritiesChangeSignaturesEvent {
        pub id: String,
        #[serde(rename = "blockNumber")]
        pub block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "blockHash")]
        pub block_hash: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub message: Vec<u8>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CollectedEnoughAuthoritiesChangeSignaturesEvent {
        pub id: String,
        #[serde(rename = "blockNumber")]
        pub block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "blockHash")]
        pub block_hash: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub message: Vec<u8>,
        pub signatures: DataWrapper<EcdsaSignature>,
        pub operation: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CollectedEnoughNewMessageRootSignaturesEvent {
        pub id: String,
        #[serde(rename = "blockNumber")]
        pub block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "blockHash")]
        pub block_hash: Vec<u8>,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        pub message: Vec<u8>,
        pub signatures: DataWrapper<EcdsaSignature>,
        #[serde(rename = "commitmentBlockNumber")]
        pub commitment_block_number: u32,
        #[serde(with = "SerHexSeq::<StrictPfx>")]
        #[serde(rename = "commitmentMessageRoot")]
        pub commitment_message_root: Vec<u8>,
        #[serde(rename = "commitmentNonce")]
        pub commitment_nonce: u32,
    }
}
