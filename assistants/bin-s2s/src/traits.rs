use std::fmt::Debug;

use bridge_s2s_traits::client::S2SClientRelay;
use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use feemarket_s2s_traits::api::FeemarketApiRelay;
use subquery::types::OriginType;
use subquery::Subquery;

use support_types::mark::ChainName;

use crate::error::BinS2SResult;

/// solo chain info
#[async_trait::async_trait]
pub trait S2SSoloChainInfo: 'static + Sync + Send + Sized + Clone + Debug {
    const CHAIN: ChainName;
    type Client: S2SClientRelay + FeemarketApiRelay;

    fn chain(&self) -> ChainName {
        Self::CHAIN
    }

    fn origin_type(&self) -> OriginType;

    fn account(&self) -> <<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId;

    async fn client(&self) -> BinS2SResult<Self::Client>;
}

pub trait SubqueryInfo: 'static + Sync + Send + Sized + Clone + Debug {
    fn subquery(&self) -> BinS2SResult<Subquery>;
}
