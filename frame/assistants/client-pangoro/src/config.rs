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

/// Pangoro subxt config
#[derive(Clone)]
pub enum PangoroSubxtConfig {}

impl subxt::Config for PangoroSubxtConfig {
    type Index = bp_pangoro::Nonce;
    type BlockNumber = bp_pangoro::BlockNumber;
    type Hash = bp_pangoro::Hash;
    type Hashing = bp_pangoro::Hashing;
    type AccountId = bp_pangoro::AccountId;
    type Address = bp_pangoro::Address;
    type Header = bp_pangoro::Header;
    type Signature = bp_pangoro::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
