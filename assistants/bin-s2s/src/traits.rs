use std::fmt::Debug;

use bridge_s2s_traits::client::{
    S2SClientRelay, S2SParaBridgeClientRelaychain, S2SParaBridgeClientSolochain,
};
use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use feemarket_s2s_traits::api::FeemarketApiRelay;
use subquery::types::OriginType;
use subquery::Subquery;

use support_types::mark::ChainName;

use crate::error::BinS2SResult;

pub trait S2SBasicChainInfo: 'static + Sync + Send + Sized + Clone + Debug {
    const CHAIN: ChainName;

    fn origin_type(&self) -> OriginType;

    fn chain(&self) -> ChainName {
        Self::CHAIN
    }
}

/// solo bridge solo chain info
#[async_trait::async_trait]
pub trait S2SSoloBridgeSoloChainInfo: S2SBasicChainInfo {
    type Client: S2SClientRelay + FeemarketApiRelay;

    fn account(&self) -> <<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId;

    async fn client(&self) -> BinS2SResult<Self::Client>;
}

/// para bridge solo chain info
#[async_trait::async_trait]
#[cfg(any(feature = "solo-with-para", feature = "para-with-para"))]
pub trait S2SParaBridgeSoloChainInfo: S2SBasicChainInfo {
    type Client: S2SParaBridgeClientSolochain + FeemarketApiRelay;

    fn account(&self) -> <<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId;

    async fn client(&self) -> BinS2SResult<Self::Client>;
}

/// para bridge relay chain info
#[async_trait::async_trait]
#[cfg(any(feature = "solo-with-para", feature = "para-with-para"))]
pub trait S2SParaBridgeRelayChainInfo: S2SBasicChainInfo {
    type Client: S2SParaBridgeClientRelaychain;

    async fn client(&self) -> BinS2SResult<Self::Client>;
}

pub trait SubqueryInfo: 'static + Sync + Send + Sized + Clone + Debug {
    fn subquery(&self) -> BinS2SResult<Subquery>;
}
