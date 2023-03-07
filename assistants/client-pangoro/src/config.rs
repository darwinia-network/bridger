use client_common_traits::subxt_darwinia_like::DarwiniaLikeExtrinsicParams;
use serde::{Deserialize, Serialize};

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
    type Index = bp_darwinia_core::Nonce;
    type BlockNumber = bp_darwinia_core::BlockNumber;
    type Hash = bp_darwinia_core::Hash;
    type Hashing = bp_darwinia_core::Hashing;
    type AccountId = bp_darwinia_core::AccountId;
    type Address = bp_darwinia_core::Address;
    type Header = bp_darwinia_core::Header;
    type Signature = bp_darwinia_core::Signature;
    type ExtrinsicParams = DarwiniaLikeExtrinsicParams;
}