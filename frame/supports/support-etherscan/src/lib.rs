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
