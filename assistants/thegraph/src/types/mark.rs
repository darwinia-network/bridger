/// Chain name
#[derive(Clone, Debug)]
pub enum LikethChain {
    Ethereum,
    Ropsten,
    Pangoro,
    Pangolin,
    Darwinia,
}

impl LikethChain {
    /// Graphql query directory
    #[allow(dead_code)]
    pub(crate) fn directory(&self) -> &str {
        match self {
            Self::Ethereum => "ethereum",
            Self::Ropsten => "ropsten",
            Self::Pangoro => "pangoro",
            Self::Pangolin => "pangolin",
            Self::Darwinia => "darwinia",
        }
    }
}
