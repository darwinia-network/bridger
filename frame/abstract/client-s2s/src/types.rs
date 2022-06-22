pub use bp_header_chain;
pub use bp_messages;
pub use bp_polkadot_core;
pub use bridge_runtime_common;
use sp_runtime::codec;

#[derive(codec::Encode, codec::Decode, Debug, Clone)]
pub struct HeadData(pub Vec<u8>);
