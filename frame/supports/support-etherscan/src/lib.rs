use std::time::Duration;

use web3::{
    api::{Eth, EthFilter, Namespace},
    confirm::wait_for_confirmations,
    types::{H256, U64},
    Transport,
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
        tracing::trace!("get_gas_oracle:{:?}", &url);
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

async fn transaction_receipt_block_number_check<T: Transport>(
    eth: &Eth<T>,
    hash: H256,
    since: u64,
    timeout: u64,
) -> web3::error::Result<Option<U64>> {
    let receipt = eth.transaction_receipt(hash).await?;
    let current_block_number = eth.block_number().await?;
    tracing::trace!(
        target: "support-etherscan",
        "hash:{:?}, current:{:?}, timeout:{:?}, since:{:?}",
        hash, current_block_number.as_u64(), timeout, since
    );
    if current_block_number.as_u64() < timeout + since {
        Ok(receipt.and_then(|receipt| receipt.block_number))
    } else {
        tracing::warn!(target: "support-etherscan", "Transaction({:?}) confirmation timeout.", hash);
        Ok(Some(since.into()))
    }
}

// Given a transaction hash, wait for confirmations.
pub async fn wait_for_transaction_confirmation_with_timeout<T: Transport>(
    hash: H256,
    transport: T,
    poll_interval: Duration,
    confirmations: usize,
    timeout: u64,
) -> web3::error::Result<()> {
    if confirmations == 0 {
        return Ok(());
    }

    let eth = Eth::new(transport.clone());
    let current = eth.block_number().await?;
    let confirmation_check =
        || transaction_receipt_block_number_check(&eth, hash, current.as_u64(), timeout);
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
