use crate::error::E2EClientResult;
use web3::types::U256;

/// relay decide
#[async_trait::async_trait]
pub trait RelayStrategy: 'static + Clone + Send + Sync {
    /// decide to relay
    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool>;
}

#[derive(Clone)]
pub struct AlwaysRelayStrategy;

#[async_trait::async_trait]
impl RelayStrategy for AlwaysRelayStrategy {
    async fn decide(&mut self, _encoded_key: U256) -> E2EClientResult<bool> {
        Ok(true)
    }
}

pub struct EnforcementRelayStrategy<Strategy: RelayStrategy> {
    strategy: Strategy,
}

impl<Strategy: RelayStrategy> EnforcementRelayStrategy<Strategy> {
    pub fn new(strategy: Strategy) -> Self {
        Self { strategy }
    }
}

impl<Strategy: RelayStrategy> EnforcementRelayStrategy<Strategy> {
    pub async fn decide(&mut self, encoded_keys: &[U256]) -> E2EClientResult<bool> {
        for key in encoded_keys {
            let result = self.strategy.decide(key.clone()).await?;
            if !result {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
