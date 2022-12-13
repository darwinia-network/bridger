use serde::{Deserialize, Serialize};
// use subxt::sp_runtime;

use subxt::{
    config::{Config, SubstrateConfig},
    tx::SubstrateExtrinsicParams,
};

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
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;

    // type Index = <SubstrateConfig as Config>::Index;
    // type BlockNumber = <SubstrateConfig as Config>::BlockNumber;
    // type Hash = <SubstrateConfig as Config>::Hash;
    // type Hashing = <SubstrateConfig as Config>::Hashing;
    // type AccountId = <SubstrateConfig as Config>::AccountId;
    // type Address = <SubstrateConfig as Config>::Address;
    // type Header = <SubstrateConfig as Config>::Header;
    // type Signature = <SubstrateConfig as Config>::Signature;
    // type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}
