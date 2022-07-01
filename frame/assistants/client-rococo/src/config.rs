use serde::{Deserialize, Serialize};
use subxt::sp_runtime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientConfig {
    pub endpoint: String,

    /// relayer's private key
    pub relayer_private_key: String,
    /// the real account behind the relayer
    pub relayer_real_account: Option<String>,
}

/// Rococo subxt config
#[derive(Clone)]
pub enum RococoSubxtConfig {}

impl subxt::Config for RococoSubxtConfig {
    type Index = bp_rococo::Nonce;
    type BlockNumber = bp_rococo::BlockNumber;
    type Hash = bp_rococo::Hash;
    type Hashing = bp_rococo::Hashing;
    type AccountId = bp_rococo::AccountId;
    type Address = bp_rococo::Address;
    type Header = bp_rococo::Header;
    type Signature = bp_rococo::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
