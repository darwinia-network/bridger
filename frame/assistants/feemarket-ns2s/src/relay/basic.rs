use bp_runtime::Chain;

use crate::api::FeemarketApi;
use crate::error::FeemarketResult;

/// Basic relay strategy
/// 1. if you are assigned relayer you will relay all order whether or not it times out
/// 2. if you aren't assigned relayer, only participate in the part about time out, earn more rewards
/// 3. if not have any assigned relayers, everyone participates in the relay.
#[derive(Clone)]
pub struct BasicRelayStrategy<A: FeemarketApi> {
    api: A,
    account: <A::Chain as Chain>::AccountId,
}

impl<A: FeemarketApi> BasicRelayStrategy<A> {
    pub fn new(api: A, account: <A::Chain as Chain>::AccountId) -> Self {
        Self { api, account }
    }
}

impl<A: FeemarketApi> BasicRelayStrategy<A> {}
