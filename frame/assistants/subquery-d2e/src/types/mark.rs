use serde::{Deserialize, Serialize};

/// Allow bridges
#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString)]
pub enum BridgeName {
    /// bridge pangolin-ropsten
    PangolinRopsten,
    /// bridge darwinia-ethereum
    DarwiniaEthereum,
}

impl BridgeName {
    /// Graphql query directory
    pub(crate) fn directory(&self) -> &str {
        match self {
            Self::PangolinRopsten => "pangolin-ropsten",
            Self::DarwiniaEthereum => "darwinia-ethereum",
        }
    }
}
