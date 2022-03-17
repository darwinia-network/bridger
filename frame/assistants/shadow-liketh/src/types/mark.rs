/// Bridge names
pub enum BridgeName {
    PangolinRopsten,
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
