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
pub enum PangolinParachainSubxtConfig {}

impl subxt::Config for PangolinParachainSubxtConfig {
    type Index = bp_pangolin_parachain::Nonce;
    type BlockNumber = bp_pangolin_parachain::BlockNumber;
    type Hash = bp_pangolin_parachain::Hash;
    type Hashing = bp_pangolin_parachain::Hashing;
    type AccountId = bp_pangolin_parachain::AccountId;
    type Address = bp_pangolin_parachain::Address;
    type Header = bp_pangolin_parachain::Header;
    type Signature = bp_pangolin_parachain::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
