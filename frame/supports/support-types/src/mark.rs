use serde::{Deserialize, Serialize};

/// Allow bridges
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BridgeName {
    /// bridge darwinia-ethereum
    #[serde(rename = "darwinia-ethereum")]
    DarwiniaEthereum,
    /// bridge pangolin<>pangolin-parachain
    #[serde(rename = "pangolin-pangolinparachain")]
    PangolinPangolinParachain,
    /// bridge pangolin<>pangolin-parachainalpha
    #[serde(rename = "pangolin-pangolinparachainalpha")]
    PangolinPangolinParachainAlpha,
    /// bridge pangolin<>pangoro
    #[serde(rename = "pangolin-pangoro")]
    PangolinPangoro,
    /// bridge crab<>crab-parachain
    #[serde(rename = "crab-crabparachain")]
    CrabCrabParachain,
    /// bridge darwinia<>crab
    #[serde(rename = "darwinia-crab")]
    DarwiniaCrab,
    /// bridge pangoro<>goerli (goerli)
    #[serde(rename = "pangoro-goerli")]
    PangoroGoerli,
    /// bridge pangoro<>goerli (goerli)
    #[serde(rename = "pangolin-goerli")]
    PangolinGoerli,
    /// bridge darwinia<>darwinia-parachain
    #[serde(rename = "darwinia-darwiniaparachain")]
    DarwiniaDarwiniaParachain,
}

impl BridgeName {
    /// Graphql query directory
    pub fn name(&self) -> &str {
        match self {
            Self::DarwiniaEthereum => "darwinia-ethereum",
            Self::PangolinPangolinParachain => "pangolin-pangolinparachain",
            Self::PangolinPangolinParachainAlpha => "pangolin-pangolinparachainalpha",
            Self::PangolinPangoro => "pangolin-pangoro",
            Self::CrabCrabParachain => "crab-crabparachain",
            Self::DarwiniaCrab => "darwinia-crab",
            Self::PangoroGoerli => "pangoro-goerli",
            Self::PangolinGoerli => "pangolin-goerli",
            Self::DarwiniaDarwiniaParachain => "darwinia-darwiniaparachain",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChainName {
    #[serde(rename = "darwinia")]
    Darwinia,
    #[serde(rename = "crab")]
    Crab,
    #[serde(rename = "pangolin")]
    Pangolin,
    #[serde(rename = "pangoro")]
    Pangoro,
    #[serde(rename = "pangolinparachain")]
    PangolinParachain,
    #[serde(rename = "pangolinparachainalpha")]
    PangolinParachainAlpha,
    #[serde(rename = "crabparachain")]
    CrabParachain,
    #[serde(rename = "darwiniaparachain")]
    DarwiniaParachain,
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "goerli")]
    Goerli,
    #[serde(rename = "rococo")]
    Rococo,
    #[serde(rename = "kusama")]
    Kusama,
    #[serde(rename = "polkadot")]
    Polkadot,
    #[serde(rename = "moonbase")]
    Moonbase,
}

impl ChainName {
    pub fn name(&self) -> &str {
        match self {
            Self::Darwinia => "darwinia",
            Self::Crab => "crab",
            Self::Pangolin => "pangolin",
            Self::Pangoro => "pangoro",
            Self::PangolinParachain => "pangolinparachain",
            Self::PangolinParachainAlpha => "pangolinparachainalpha",
            Self::CrabParachain => "crabparachain",
            Self::DarwiniaParachain => "darwiniaparachain",
            Self::Ethereum => "ethereum",
            Self::Goerli => "goerli",
            Self::Rococo => "rococo",
            Self::Kusama => "kusama",
            Self::Polkadot => "polkadot",
            Self::Moonbase => "moonbase",
        }
    }
}
