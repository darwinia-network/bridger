use std::str::FromStr;

use secp256k1::SecretKey;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

#[allow(dead_code)]
pub struct FeeMarket {
    pub contract: Contract<Http>,
}

impl FeeMarket {
    #[allow(dead_code)]
    pub fn new(client: Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("FeeMarket.json"),
        )?;
        Ok(Self { contract })
    }

    #[allow(dead_code)]
    pub async fn enroll(
        &self,
        prev: Address,
        fee: U256,
        private_key: &SecretKey,
    ) -> color_eyre::Result<H256> {
        let tx = self
            .contract
            .signed_call(
                "enroll",
                (prev, fee),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    value: Some(fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?;
        Ok(tx)
    }

    #[allow(dead_code)]
    pub async fn deposit(&self, fee: U256, private_key: &SecretKey) -> color_eyre::Result<H256> {
        let tx = self
            .contract
            .signed_call(
                "deposit",
                (),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    value: Some(fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?;
        Ok(tx)
    }
}

pub mod types {}

#[cfg(test)]
mod tests {
    use web3::ethabi::Token;

    use super::*;

    fn test_fee_market() -> FeeMarket {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        FeeMarket::new(client, "0x721F10bdE716FF44F596Afa2E8726aF197e6218E").unwrap()
    }

    #[ignore]
    #[tokio::test]
    async fn test_enroll() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let fee_market =
            FeeMarket::new(client, "0x721F10bdE716FF44F596Afa2E8726aF197e6218E").unwrap();
        let private_key = SecretKey::from_str("//Alice").unwrap();
        let tx = fee_market
            .enroll(
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                U256::from(100000000000000u64),
                &private_key,
            )
            .await
            .unwrap();
        println!("{:?}", tx);
    }

    #[ignore]
    #[tokio::test]
    async fn test_deposit() {
        let fee_market = test_fee_market();
        let private_key = SecretKey::from_str("//Alice").unwrap();
        let tx = fee_market
            .deposit(U256::from(100000000000000u64), &private_key)
            .await
            .unwrap();
        println!("{:?}", tx);
    }

    #[ignore]
    #[tokio::test]
    async fn test_query() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let fee_market =
            FeeMarket::new(client, "0x721F10bdE716FF44F596Afa2E8726aF197e6218E").unwrap();
        let r: Token = fee_market
            .contract
            .query(
                "getTopRelayers",
                (),
                None,
                Options {
                    gas: Some(U256::from(10000000u64)),
                    gas_price: Some(U256::from(1300000000u64)),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();
        println!("{:?}", r);
    }
}
