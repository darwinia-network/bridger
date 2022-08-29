use serde::{Deserialize, Serialize};

/// Allow bridges
#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString)]
pub enum BridgeName {
    /// bridge pangolin-ropsten
    #[cfg(feature = "bridge-ethv1")]
    PangolinRopsten,
    /// bridge darwinia-ethereum
    #[cfg(feature = "bridge-ethv1")]
    DarwiniaEthereum,
    /// bridge pangolin<>pangolin-parachain
    #[cfg(feature = "bridge-s2s")]
    PangolinParachain,
    /// bridge pangolin<>pangoro
    #[cfg(feature = "bridge-s2s")]
    PangolinPangoro,
    /// bridge pangolin<>pangolin-parachain
    #[cfg(feature = "bridge-s2s")]
    CrabParachain,
    /// bridge darwinia<>crab
    #[cfg(feature = "bridge-s2s")]
    DarwiniaCrab,
    /// bridge pangoro<>goerli (goerli)
    #[cfg(feature = "bridge-ethv2")]
    PangoroGoerli,
}

impl BridgeName {
    /// Graphql query directory
    pub(crate) fn directory(&self) -> &str {
        match self {
            #[cfg(feature = "bridge-ethv1")]
            Self::PangolinRopsten => "pangolin-ropsten",
            #[cfg(feature = "bridge-ethv1")]
            Self::DarwiniaEthereum => "darwinia-ethereum",
            #[cfg(feature = "bridge-s2s")]
            Self::PangolinParachain => "pangolin-parachain",
            #[cfg(feature = "bridge-s2s")]
            Self::PangolinPangoro => "pangolin-pangoro",
            #[cfg(feature = "bridge-s2s")]
            Self::CrabParachain => "crab-parachain",
            #[cfg(feature = "bridge-s2s")]
            Self::DarwiniaCrab => "darwinia-crab",
            #[cfg(feature = "bridge-ethv2")]
            Self::PangoroGoerli => "pangoro-goerli",
        }
    }
}
