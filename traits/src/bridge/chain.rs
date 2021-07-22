pub trait BridgeChain {
    const CHAIN_CATEGORY: ChainCategory;
}

pub trait SubstrateChain: BridgeChain {
    type ChainTypes;
}

pub trait LikeDarwiniaChain: SubstrateChain {}

pub trait LikeEthereumChain: BridgeChain {}

pub enum ChainCategory {
    Substrate,
    Ethereum,
}
