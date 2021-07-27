use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, bridge_traits::BridgeCrypto)]
pub struct DarwiniaSubxtConfig {
    /// is enable crypto private key
    #[crypto(is_enable)]
    enable_crypto: bool,
    pub endpoint: String,

    /// relayer's private key
    #[crypto(decrypt)]
    pub relayer_private_key: String,
    /// the real account behind the relayer
    pub relayer_real_account: Option<String>,

    /// private key to sign ecdsa messages, the signature will be submitted to Darwinia by relayer
    pub ecdsa_authority_private_key: Option<String>,
}

impl BridgeConfig for DarwiniaSubxtConfig {
    fn marker() -> &'static str {
        "component-darwinia-subxt"
    }

    fn template() -> Self {
        Self {
            enable_crypto: false,
            endpoint: "wss://rpc.darwinia.network".to_string(),
            relayer_private_key: "0x...".to_string(),
            relayer_real_account: Some("0x...".to_string()),
            ecdsa_authority_private_key: Some("0x...".to_string()),
        }
    }
}
