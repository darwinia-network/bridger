//! Ethereum HeaderThing
use crate::{
    bytes,
    chain::eth::{EthereumHeader, EthereumHeaderJson},
    hex,
};
use codec::{Decode, Encode};

/// Ethereum HeaderThing
#[derive(Encode, Decode, Debug, Default)]
pub struct HeaderThing {
    /// Ethereum header
    pub header: EthereumHeader,
    /// MMR root
    pub mmr_root: [u8; 32],
}

/// Ethereum HeaderThing JSON
#[derive(Default, Deserialize)]
pub struct HeaderThingJson {
    /// Ethereum header
    pub header: EthereumHeaderJson,
    /// MMR root
    pub mmr_root: String,
}

impl Into<HeaderThing> for HeaderThingJson {
    fn into(self) -> HeaderThing {
        HeaderThing {
            header: self.header.into(),
            mmr_root: bytes!(self.mmr_root.as_str(), 32),
        }
    }
}

impl Into<HeaderThingJson> for HeaderThing {
    fn into(self) -> HeaderThingJson {
        HeaderThingJson {
            header: self.header.into(),
            mmr_root: hex!(&self.mmr_root),
        }
    }
}

/// Ethereum HeaderThing with proof JSON
#[derive(Default, Deserialize)]
pub struct HeaderThingWithConfirmationJson {
    /// Ethereum header
    pub header_thing: HeaderThingJson,
    /// MMR root
    pub confirmation: u64,
}
