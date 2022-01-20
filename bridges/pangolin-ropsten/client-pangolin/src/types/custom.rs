use codec::{Decode, Encode};

/// Substrate balance type
pub type Balance = u128;

/// EcdsaSignature
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct EcdsaSignature(pub [u8; 65]);

impl Default for EcdsaSignature {
    fn default() -> Self {
        Self([0u8; 65])
    }
}

/// EcdsaAddress
pub type EcdsaAddress = [u8; 20];

/// EcdsaMessage
pub type EcdsaMessage = [u8; 32];
