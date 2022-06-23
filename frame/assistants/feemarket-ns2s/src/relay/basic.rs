use abstract_bridge_s2s::client::{S2SClientGeneric, S2SClientRelay};
use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::error::S2SClientResult;
use abstract_bridge_s2s::strategy::{RelayReference, RelayStrategy};

use crate::api::FeemarketApi;
use crate::error::FeemarketResult;

/// Basic relay strategy
/// 1. if you are assigned relayer you will relay all order whether or not it times out
/// 2. if you aren't assigned relayer, only participate in the part about time out, earn more rewards
/// 3. if not have any assigned relayers, everyone participates in the relay.
pub struct BasicRelayStrategy<A, C>
where
    A: FeemarketApi<C>,
    C: S2SClientGeneric,
{
    api: A,
    account: <C::Config as Config>::AccountId,
}

impl<A, C> BasicRelayStrategy<A, C>
where
    A: FeemarketApi<C>,
    C: S2SClientGeneric,
{
    pub fn new(api: A, account: <C::Config as Config>::AccountId) -> Self {
        Self { api, account }
    }
}

impl<A, C> BasicRelayStrategy<A, C>
where
    A: FeemarketApi<C>,
    C: S2SClientGeneric,
{
}

impl<A, SC> Clone for BasicRelayStrategy<A, SC>
where
    A: FeemarketApi<SC>,
    SC: S2SClientRelay,
{
    fn clone(&self) -> Self {
        Self {
            api: self.api.clone(),
            account: self.account.clone(),
        }
    }
}

#[async_trait::async_trait]
impl<A, SC, TC> RelayStrategy<SC, TC> for BasicRelayStrategy<A, SC>
where
    A: FeemarketApi<SC>,
    SC: S2SClientRelay,
    TC: S2SClientRelay,
{
    async fn decide(
        &mut self,
        reference: RelayReference<'async_trait, SC, TC>,
    ) -> S2SClientResult<bool> {
        Ok(true)
    }
}
