use codec::{Decode, Encode};

/// EcdsaAddress
pub type EcdsaAddress = [u8; 20];

/// EcdsaSignature
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct EcdsaSignature(pub [u8; 65]);

impl Default for EcdsaSignature {
	fn default() -> Self {
		Self([0u8; 65])
	}
}

/// EcdsaMessage
pub type EcdsaMessage = [u8; 32];
