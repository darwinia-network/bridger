use std::cmp;
use std::time::Duration;

use web3::{
    api::{Eth, EthFilter, Namespace},
    confirm::wait_for_confirmations,
    transports::Http,
    types::{H256, U256, U64},
    Transport, Web3,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, value::Value};

mod error;
pub use error::{Error, Result};

pub struct EtherscanClient {
    client: Client,
    api_key: String,
}

// The gas prices units are Gwei.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasOracle {
    #[serde(rename = "LastBlock")]
    pub last_block: String,
    #[serde(rename = "SafeGasPrice")]
    pub safe_gas_price: String,
    #[serde(rename = "ProposeGasPrice")]
    pub propose_gas_price: String,
    #[serde(rename = "FastGasPrice")]
    pub fast_gas_price: String,
    #[serde(rename = "suggestBaseFee")]
    pub suggest_base_fee: String,
    #[serde(rename = "gasUsedRatio")]
    pub gas_used_ratio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}
impl EtherscanClient {
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            api_key: api_key.into(),
        })
    }

    // The gas prices are returned in Gwei.
    pub async fn get_gas_oracle(&self) -> Result<GasOracle> {
        let url = format!(
            "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey={api_key}",
            api_key = self.api_key,
        );
        dbg!(&url);
        let response: ApiResult<Value> = self.client.get(url).send().await?.json().await?;
        if response.status == "1" {
            Ok(from_value(response.result)?)
        } else {
            Err(Error::Etherscan(format!(
                "Failed to get gas orcale {}",
                response.message
            )))
        }
    }
}

#[async_trait::async_trait]
pub trait GasPriceOracle {
    fn get_web3(&self) -> &Web3<Http>;
    fn get_etherscan_client(&self) -> Option<&EtherscanClient>;
    fn max_gas_price(&self) -> U256;
    async fn gas_price(&self) -> Result<U256> {
        let price: U256 = match self.get_etherscan_client() {
            Some(etherscan_client) => {
                let oracle = etherscan_client.get_gas_oracle().await?;
                U256::from_dec_str(&oracle.propose_gas_price)? * 1_000_000_000i64
            }
            None => self.get_web3().eth().gas_price().await?,
        };
        Ok(cmp::min(self.max_gas_price(), price))
    }
}

async fn transaction_receipt_block_number_check<T: Transport>(
    eth: &Eth<T>,
    hash: H256,
) -> web3::error::Result<Option<U64>> {
    let receipt = eth.transaction_receipt(hash).await?;
    Ok(receipt.and_then(|receipt| receipt.block_number))
}

// Given a transaction hash, wait for confirmations.
pub async fn wait_for_transaction_confirmation<T: Transport>(
    hash: H256,
    transport: T,
    poll_interval: Duration,
    confirmations: usize,
) -> web3::error::Result<()> {
    let eth = Eth::new(transport.clone());
    if confirmations > 0 {
        let confirmation_check = || transaction_receipt_block_number_check(&eth, hash);
        let eth_filter = EthFilter::new(transport.clone());
        let eth = eth.clone();
        wait_for_confirmations(
            eth,
            eth_filter,
            poll_interval,
            confirmations,
            confirmation_check,
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::EtherscanClient;

    #[tokio::test]
    async fn test_get_gas_oracle() {
        let client = EtherscanClient::new("").unwrap();
        let result = client.get_gas_oracle().await.unwrap();
        dbg!(result);
    }
}
