pub use bp_header_chain;
pub use bp_messages;
pub use bridge_runtime_common;
use serde::{Deserialize, Serialize};
use sp_core::Hasher as HasherT;
use sp_runtime::codec;
use sp_runtime::traits::BlakeTwo256;

type ParaHash = <BlakeTwo256 as HasherT>::Out;

#[derive(codec::Encode, codec::Decode, Debug, Clone, Deserialize, Serialize)]
pub struct ParaId(pub u32);

#[derive(codec::Encode, codec::Decode, Debug, Clone, Deserialize, Serialize)]
pub struct HeadData(pub Vec<u8>);

/// Best known parachain head as it is stored in the runtime storage.
#[derive(codec::Encode, codec::Decode, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BestParaHead {
    /// Number of relay block where this head has been updated.
    pub at_relay_block_number: u32,
    /// Hash of parachain head.
    pub head_hash: ParaHash,
    /// Current ring buffer position for this parachain.
    pub next_imported_hash_position: u32,
}
