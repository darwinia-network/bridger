// use serde::Deserialize;

/// Substrate balance type
pub type Balance = u128;

/// EcdsaAddress
pub type EcdsaAddress = [u8; 20];

/// EcdsaMessage
pub type EcdsaMessage = [u8; 32];

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct HeaderMMRRpc {
//     mmr_size: String,
//     proof: String,
// }