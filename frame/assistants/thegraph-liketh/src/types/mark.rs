/// Chain name
pub enum LikethChain {
    Ethereum,
    Ropsten,
}

impl LikethChain {
    /// Graphql query directory
    pub(crate) fn directory(&self) -> &str {
        match self {
            Self::Ethereum => "ethereum",
            Self::Ropsten => "ropsten",
        }
    }
}
