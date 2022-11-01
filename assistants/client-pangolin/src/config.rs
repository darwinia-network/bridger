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

/// Pangolin subxt config
#[derive(Clone)]
pub enum PangolinSubxtConfig {}

impl subxt::Config for PangolinSubxtConfig {
    type Index = bp_pangolin::Nonce;
    type BlockNumber = bp_pangolin::BlockNumber;
    type Hash = bp_pangolin::Hash;
    type Hashing = bp_pangolin::Hashing;
    type AccountId = bp_pangolin::AccountId;
    type Address = bp_pangolin::Address;
    type Header = bp_pangolin::Header;
    type Signature = bp_pangolin::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
