pub use bp_header_chain;
pub use bp_messages;
pub use bp_runtime;
pub use bridge_runtime_common;

#[cfg(feature = "bridge-parachain")]
pub use self::bridge_parachain::*;

#[cfg(feature = "bridge-parachain")]
mod bridge_parachain {
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
    pub struct ParaInfo {
        pub best_head_hash: BestParaHeadHash,
        /// Current ring buffer position for this parachain.
        pub next_imported_hash_position: u32,
    }

    #[derive(codec::Encode, codec::Decode, Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BestParaHeadHash {
        pub at_relay_block_number: u32,
        pub head_hash: ParaHash,
    }

    impl HeadData {
        /// Returns the hash of this head data.
        pub fn hash(&self) -> sp_core::H256 {
            sp_runtime::traits::BlakeTwo256::hash(&self.0)
        }
    }
}
