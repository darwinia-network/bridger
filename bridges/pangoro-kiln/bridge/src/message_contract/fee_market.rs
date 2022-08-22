use std::time::{SystemTime, UNIX_EPOCH};

use bridge_e2e_traits::{
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use client_contracts::FeeMarket;
use web3::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct FeeMarketRelayStrategy {
    fee_market: FeeMarket,
    account: Address,
}

impl FeeMarketRelayStrategy {
    pub fn new(fee_market: FeeMarket, account: Address) -> Self {
        Self {
            fee_market,
            account,
        }
    }
}

#[async_trait::async_trait]
impl RelayStrategy for FeeMarketRelayStrategy {
    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool> {
        let (order, exts) = self
            .fee_market
            .order(encoded_key)
            .await
            .map_err(|e| E2EClientError::Custom(format!("[feemarket]: {:?}", e)))?;

        let is_assigned_relayer = exts.iter().any(|x| x.assigned_relayer == self.account);
        if is_assigned_relayer {
            tracing::info!(
                target: "feemarket",
                "[feemarket] You are assigned relayer, you must relay this message: {:?}",
                encoded_key
            );
            return Ok(true);
        }

        let relay_time = self
            .fee_market
            .relay_time()
            .await
            .map_err(|e| E2EClientError::Custom(format!("[feemarket]: {:?}", e)))?;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| E2EClientError::Custom(format!("[feemarket]: {:?}", e)))?
            .as_secs();
        if current_time - order.assigned_time as u64 > relay_time {
            tracing::info!(
                target: "feemarket",
                "[feemarket] You aren't assigned relayer, but this message is timeout. Decide to relay this message"
            );
            return Ok(true);
        }

        tracing::info!(
            target: "feemarket",
            "[feemarket] You aren't assigned relayer, and nonce({:?}) is on-time. so don't relay this",
            encoded_key
        );

        Ok(false)
    }
}
