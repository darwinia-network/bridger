use serde::{Deserialize, Serialize};
use subxt::{sp_core, sp_runtime, StorageEntry};

use crate::codegen::api::DefaultAccountData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientConfig {
    pub endpoint: String,

    /// relayer's private key
    pub relayer_private_key: String,
    /// the real account behind the relayer
    pub relayer_real_account: Option<String>,

    /// private key to sign ecdsa messages, the signature will be submitted to Darwinia by relayer
    pub ecdsa_authority_private_key: Option<String>,
}

/// Pangolin subxt config
#[derive(Clone)]
pub enum PangolinSubxtConfig {}

impl subxt::Config for PangolinSubxtConfig {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = sp_runtime::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, sp_runtime::traits::BlakeTwo256>;
    type Signature = sp_runtime::MultiSignature;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

impl subxt::AccountData<PangolinSubxtConfig> for DefaultAccountData {
    fn storage_entry(account_id: <PangolinSubxtConfig as subxt::Config>::AccountId) -> Self {
        Self(account_id)
    }

    fn nonce(
        result: &<Self as StorageEntry>::Value,
    ) -> <PangolinSubxtConfig as subxt::Config>::Index {
        result.nonce
    }
}
