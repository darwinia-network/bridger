use serde::{Deserialize, Serialize};
use subxt::tx::SubstrateExtrinsicParams;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientConfig {
    pub endpoint: String,
    /// relayer's private key
    pub relayer_private_key: String,
}

/// Moonbase subxt config
#[derive(Clone)]
pub enum MoonbaseSubxtConfig {}

impl subxt::Config for MoonbaseSubxtConfig {
    type Index = bp_polkadot_core::Nonce;
    type BlockNumber = bp_polkadot_core::BlockNumber;
    type Hash = bp_polkadot_core::Hash;
    type Hashing = bp_polkadot_core::Hashing;
    type AccountId = bp_polkadot_core::AccountId;
    type Address = bp_polkadot_core::Address;
    type Header = bp_polkadot_core::Header;
    type Signature = bp_polkadot_core::Signature;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}
