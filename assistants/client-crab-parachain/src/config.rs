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
pub enum CrabParachainSubxtConfig {}

impl subxt::Config for CrabParachainSubxtConfig {
    type Index = bp_crab_parachain::Nonce;
    type BlockNumber = bp_crab_parachain::BlockNumber;
    type Hash = bp_crab_parachain::Hash;
    type Hashing = bp_crab_parachain::Hashing;
    type AccountId = bp_crab_parachain::AccountId;
    type Address = bp_crab_parachain::Address;
    type Header = bp_crab_parachain::Header;
    type Signature = bp_crab_parachain::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
