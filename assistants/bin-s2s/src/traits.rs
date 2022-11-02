use std::fmt::Debug;

use bridge_s2s_traits::client::S2SClientRelay;
use subquery::types::OriginType;
use subquery::Subquery;

use support_types::mark::ChainName;

use crate::error::BinS2SResult;

/// solo chain info
#[async_trait::async_trait]
pub trait S2SSoloChainInfo: 'static + Sync + Send + Sized + Clone + Debug {
    const CHAIN: ChainName;
    type Client: S2SClientRelay;

    fn chain(&self) -> ChainName {
        Self::CHAIN
    }

    async fn client(&self) -> BinS2SResult<Self::Client>;

    fn origin_type(&self) -> OriginType;
}

pub trait SubqueryInfo: 'static + Sync + Send + Sized + Clone + Debug {
    fn subquery(&self) -> BinS2SResult<Subquery>;
}
