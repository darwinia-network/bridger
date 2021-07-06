use bridge_traits::bridge::chain::{BridgeChain, ChainCategory, LikeEthereumChain};

#[derive(Clone, Debug)]
pub struct EthereumChain {}

impl BridgeChain for EthereumChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Ethereum;
}

impl LikeEthereumChain for EthereumChain {}
