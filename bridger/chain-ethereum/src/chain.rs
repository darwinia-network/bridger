use bridge_standard::bridge::chain::{BridgeChain, ChainCategory, LikeEthereumChain};

pub struct EthereumChain {}

impl BridgeChain for EthereumChain {
    const CHAIN_CATEGORY: ChainCategory = ChainCategory::Ethereum;
}

impl LikeEthereumChain for EthereumChain {}
