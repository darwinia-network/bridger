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
    type Index = bp_darwinia_core::Nonce;
    type BlockNumber = bp_darwinia_core::BlockNumber;
    type Hash = bp_darwinia_core::Hash;
    type Hashing = bp_darwinia_core::Hashing;
    type AccountId = bp_darwinia_core::AccountId;
    type Address = bp_darwinia_core::Address;
    type Header = bp_darwinia_core::Header;
    type Signature = bp_darwinia_core::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
