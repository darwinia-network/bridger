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

/// Crab subxt config
#[derive(Clone)]
pub enum CrabSubxtConfig {}

impl subxt::Config for CrabSubxtConfig {
    type Index = bp_crab::Nonce;
    type BlockNumber = bp_crab::BlockNumber;
    type Hash = bp_crab::Hash;
    type Hashing = bp_crab::Hashing;
    type AccountId = bp_crab::AccountId;
    type Address = bp_crab::Address;
    type Header = bp_crab::Header;
    type Signature = bp_crab::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
