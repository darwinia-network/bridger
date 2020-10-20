//! Ethereum EthereumHeaderThing
use crate::{
    bytes,
    chain::eth::{EthereumHeader, EthereumHeaderJson},
    hex,
};
use codec::{Decode, Encode};

/// Ethereum EthereumHeaderThing
#[derive(Encode, Decode, Debug, Default, PartialEq, Eq)]
pub struct EthereumHeaderThing {
    /// Ethereum header
    pub header: EthereumHeader,
    /// MMR root
    pub mmr_root: [u8; 32],
}

/// Ethereum EthereumHeaderThing JSON
#[derive(Default, Deserialize)]
pub struct EthereumHeaderThingJson {
    /// Ethereum header
    pub header: EthereumHeaderJson,
    /// MMR root
    pub mmr_root: String,
}

impl Into<EthereumHeaderThing> for EthereumHeaderThingJson {
    fn into(self) -> EthereumHeaderThing {
        EthereumHeaderThing {
            header: self.header.into(),
            mmr_root: bytes!(self.mmr_root.as_str(), 32),
        }
    }
}

impl Into<EthereumHeaderThingJson> for EthereumHeaderThing {
    fn into(self) -> EthereumHeaderThingJson {
        EthereumHeaderThingJson {
            header: self.header.into(),
            mmr_root: hex!(&self.mmr_root),
        }
    }
}

/// Ethereum EthereumHeaderThing with proof JSON
#[derive(Default, Deserialize)]
pub struct EthereumHeaderThingWithConfirmationJson {
    /// Ethereum header
    pub header_thing: EthereumHeaderThingJson,
    /// MMR root
    pub confirmation: u64,
}
