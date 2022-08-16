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
    use serde_hex::SerHexOpt;
    use serde_hex::SerHexSeq;
    use serde_hex::StrictPfx;

    use crate::types::DataWrapper;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum AOperationType {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "swap")]
        Swap,
    }

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
        #[serde(rename = "operationType")]
        pub operation_type: AOperationType,
        #[serde(with = "SerHexOpt::<StrictPfx>")]
        #[serde(rename = "operationNew")]
        pub operation_new: Option<[u8; 20]>,
        #[serde(with = "SerHexOpt::<StrictPfx>")]
        #[serde(rename = "operationOld")]
        pub operation_old: Option<[u8; 20]>,
        #[serde(with = "SerHexOpt::<StrictPfx>")]
        #[serde(rename = "operationPre")]
        pub operation_pre: Option<[u8; 20]>,
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
