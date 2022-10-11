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

/// Darwinia subxt config
#[derive(Clone)]
pub enum DarwiniaSubxtConfig {}

impl subxt::Config for DarwiniaSubxtConfig {
    type Index = bp_darwinia::Nonce;
    type BlockNumber = bp_darwinia::BlockNumber;
    type Hash = bp_darwinia::Hash;
    type Hashing = bp_darwinia::Hashing;
    type AccountId = bp_darwinia::AccountId;
    type Address = bp_darwinia::Address;
    type Header = bp_darwinia::Header;
    type Signature = bp_darwinia::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
