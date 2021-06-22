use bee_client::types::client::ChainTypes;

pub trait BridgeChain {
    const CHAIN_CATEGORY: ChainCategory;
}

pub trait SubstrateChain: BridgeChain {
    type ChainTypes: ChainTypes;
}

pub trait LikeEthereumChain: BridgeChain {}

pub enum ChainCategory {
    Substrate,
    Ethereum,
}
