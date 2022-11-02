use std::fmt::Debug;

use bridge_s2s_traits::client::S2SClientRelay;
use codec::{Codec, Decode, Encode, EncodeLike};
use sp_runtime::traits::{Extrinsic, MaybeSerializeDeserialize};
use subquery::types::OriginType;
use subquery::Subquery;

use support_types::mark::ChainName;

use crate::error::BinS2SResult;

/// solo chain info
#[async_trait::async_trait]
pub trait S2SSoloChainInfo: 'static + Sync + Send + Sized + Clone + Debug {
    const CHAIN: ChainName;
    type Chain: bridge_s2s_traits::types::bp_runtime::Chain;
    type Extrinsic: Codec + EncodeLike + Clone + Eq + Extrinsic + Debug + MaybeSerializeDeserialize;
    type InitializationData: Encode + Decode;

    fn chain(&self) -> ChainName {
        Self::CHAIN
    }

    async fn client(
        &self,
    ) -> BinS2SResult<
        Box<
            dyn S2SClientRelay<
                Chain = Self::Chain,
                Extrinsic = Self::Extrinsic,
                InitializationData = Self::InitializationData,
            >,
        >,
    >;

    fn origin_type(&self, source_chain: &'static str) -> OriginType;
}

pub trait SubqueryInfo: 'static + Sync + Send + Sized + Clone + Debug {
    fn subquery(&self) -> BinS2SResult<Subquery>;
}
