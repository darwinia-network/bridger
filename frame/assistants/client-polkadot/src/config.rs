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

/// Polkadot subxt config
#[derive(Clone)]
pub enum PolkadotSubxtConfig {}

impl subxt::Config for PolkadotSubxtConfig {
    type Index = bp_polkadot::Nonce;
    type BlockNumber = bp_polkadot::BlockNumber;
    type Hash = bp_polkadot::Hash;
    type Hashing = bp_polkadot::Hashing;
    type AccountId = bp_polkadot::AccountId;
    type Address = bp_polkadot::Address;
    type Header = bp_polkadot::Header;
    type Signature = bp_polkadot::Signature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}
