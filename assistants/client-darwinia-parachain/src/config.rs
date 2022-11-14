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
pub enum DarwiniaParachainSubxtConfig {}

impl subxt::Config for DarwiniaParachainSubxtConfig {
    type Index = bp_darwinia_parachain::Nonce;
    type BlockNumber = bp_darwinia_parachain::BlockNumber;
    type Hash = bp_darwinia_parachain::Hash;
    type Hashing = bp_darwinia_parachain::Hashing;
    type AccountId = bp_darwinia_parachain::AccountId;
    type Address = bp_darwinia_parachain::Address;
    type Header = bp_darwinia_parachain::Header;
    type Signature = bp_darwinia_parachain::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
