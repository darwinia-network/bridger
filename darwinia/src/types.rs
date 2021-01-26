use codec::{Decode, Encode};

/// EcdsaSignature
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct EcdsaSignature(pub [u8; 65]);

impl Default for EcdsaSignature {
	fn default() -> Self {
		Self([0u8; 65])
	}
}

