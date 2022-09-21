use std::time::{SystemTime, UNIX_EPOCH};

use bridge_e2e_traits::{
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use client_contracts::SimpleFeeMarket;
use web3::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct SimpleFeeMarketRelayStrategy {
    pub fee_market: SimpleFeeMarket,
    account: Address,
}

impl SimpleFeeMarketRelayStrategy {
    pub fn new(fee_market: SimpleFeeMarket, account: Address) -> Self {
        Self {
            fee_market,
            account,
        }
    }
}

#[async_trait::async_trait]
impl RelayStrategy for SimpleFeeMarketRelayStrategy {
    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool> {
        let order = self
            .fee_market
            .order(encoded_key)
            .await
            .map_err(|e| E2EClientError::Custom(format!("[feemarket]: {:?}", e)))?;

        let is_assigned_relayer = order.assigned_relayer == self.account;
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

pub mod types {
    use web3::contract::tokens::Detokenize;
    use web3::contract::Error;
    use web3::ethabi::Token;
    use web3::types::{Address, U256};

    #[derive(Debug, Clone)]
    pub struct Order {
        pub assigned_time: u32,
        pub assigned_relayer: Address,
        pub collateral: U256,
        pub market_fee: U256,
    }

    impl Detokenize for Order {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let (assigned_time, assigned_relayer, collateral, market_fee) =
                Detokenize::from_tokens(tokens)?;
            Ok(Self {
                assigned_time,
                assigned_relayer,
                collateral,
                market_fee,
            })
        }
    }
}
