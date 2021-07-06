use bridge_traits::bridge::chain::{BridgeChain, ChainCategory, LikeEthereumChain};

#[derive(Clone, Debug)]
pub struct TemplateChain {}

impl BridgeChain for TemplateChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Ethereum;
}

impl LikeEthereumChain for TemplateChain {}
