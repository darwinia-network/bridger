//! Ethereum EthereumRelayHeaderParcel
use crate::{
	bytes,
	chain::ethereum::{EthereumHeader, EthereumHeaderJson},
	hex,
};
use codec::{Decode, Encode};

#[cfg(feature = "runtime")]
use uint::static_assertions::_core::fmt::Formatter;

#[cfg(feature = "runtime")]
use substrate_subxt::sp_core::bytes::to_hex;

/// Ethereum EthereumRelayHeaderParcel
#[derive(Encode, Decode, Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct EthereumRelayHeaderParcel {
	/// Ethereum header
	pub header: EthereumHeader,
	/// MMR root
	pub mmr_root: [u8; 32],
}

impl EthereumRelayHeaderParcel {
	/// Is same as another parcel
	pub fn is_same_as(&self, another: &EthereumRelayHeaderParcel) -> bool {
		self.header.hash == another.header.hash && self.mmr_root == another.mmr_root
	}
}

#[cfg(feature = "runtime")]
impl std::fmt::Display for EthereumRelayHeaderParcel {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let header = &self.header.to_string();
		let msg = format!(
			"{}\n{:>19}{}",
			header,
			"mmr_root: ",
			to_hex(&self.mmr_root, false)
		);
		write!(f, "{}", msg)
	}
}

/// Ethereum EthereumRelayHeaderParcel JSON
#[derive(Default, Deserialize, Serialize)]
pub struct EthereumRelayHeaderParcelJson {
	/// Ethereum header
	pub header: EthereumHeaderJson,
	/// MMR root
	pub mmr_root: String,
}

impl Into<EthereumRelayHeaderParcel> for EthereumRelayHeaderParcelJson {
	fn into(self) -> EthereumRelayHeaderParcel {
		EthereumRelayHeaderParcel {
			header: self.header.into(),
			mmr_root: bytes!(self.mmr_root.as_str(), 32),
		}
	}
}

impl Into<EthereumRelayHeaderParcelJson> for EthereumRelayHeaderParcel {
	fn into(self) -> EthereumRelayHeaderParcelJson {
		EthereumRelayHeaderParcelJson {
			header: self.header.into(),
			mmr_root: hex!(&self.mmr_root),
		}
	}
}
