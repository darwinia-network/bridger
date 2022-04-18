use serde::{Deserialize, Serialize};

/// Allow bridges
#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString)]
pub enum BridgeName {
    /// bridge pangolin<>pangolin-parachain
    PangolinParachain,
}

impl BridgeName {
    /// Graphql query directory
    pub(crate) fn directory(&self) -> &str {
        match self {
            Self::PangolinParachain => "pangolin-parachain",
        }
    }
}
