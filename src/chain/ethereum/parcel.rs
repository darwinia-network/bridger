//! Ethereum EthereumRelayHeaderParcel
use crate::{
    bytes,
    chain::ethereum::{EthereumHeader, EthereumHeaderJson},
    hex,
};
use codec::{Decode, Encode};

/// Ethereum EthereumRelayHeaderParcel
#[derive(Encode, Decode, Debug, Default, PartialEq, Eq, Clone)]
pub struct EthereumRelayHeaderParcel {
    /// Ethereum header
    pub header: EthereumHeader,
    /// MMR root
    pub mmr_root: [u8; 32],
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
