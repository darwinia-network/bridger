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

/// Kusama subxt config
#[derive(Clone)]
pub enum KusamaSubxtConfig {}

impl subxt::Config for KusamaSubxtConfig {
    type Index = bp_kusama::Nonce;
    type BlockNumber = bp_kusama::BlockNumber;
    type Hash = bp_kusama::Hash;
    type Hashing = bp_kusama::Hashing;
    type AccountId = bp_kusama::AccountId;
    type Address = bp_kusama::Address;
    type Header = bp_kusama::Header;
    type Signature = bp_kusama::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
