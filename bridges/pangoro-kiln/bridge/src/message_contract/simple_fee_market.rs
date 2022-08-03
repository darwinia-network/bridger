use std::str::FromStr;

use crate::message_contract::simple_fee_market::types::Order;
use bridge_e2e_traits::{
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use secp256k1::SecretKey;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimpleFeeMarket {
    pub contract: Contract<Http>,
}

impl SimpleFeeMarket {
    #[allow(dead_code)]
    pub fn new(client: &Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("SimpleFeeMarket.json"),
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

    pub async fn order(&self, key: U256) -> color_eyre::Result<Order> {
        Ok(self
            .contract
            .query("orderOf", (key,), None, Options::default(), None)
            .await?)
    }
}

#[derive(Debug, Clone)]
pub struct SimpleFeeMarketRelayStrategy {
    fee_market: SimpleFeeMarket,
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
                "[feemarket] You are assigned relayer, you must be relay this message: {:?}",
                encoded_key
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

#[cfg(test)]
mod tests {
    use web3::ethabi::{RawLog, Token};
    use web3::types::{BlockNumber, FilterBuilder};

    use super::*;

    fn test_fee_market() -> (Web3<Http>, SimpleFeeMarket) {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let fee_market =
            SimpleFeeMarket::new(&client, "0x721F10bdE716FF44F596Afa2E8726aF197e6218E").unwrap();
        (client, fee_market)
    }

    #[ignore]
    #[tokio::test]
    async fn test_enroll() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let fee_market =
            SimpleFeeMarket::new(&client, "0x721F10bdE716FF44F596Afa2E8726aF197e6218E").unwrap();
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
        let (_, fee_market) = test_fee_market();
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
        let (client, fee_market) = test_fee_market();
        let r: Token = fee_market
            .contract
            .query("getTopRelayer", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn test_query_order() {
        let (_, fee_market) = test_fee_market();
        let order = fee_market
            .order(U256::from_dec_str("1461501637330902918203684832734729763729642094599").unwrap())
            .await
            .unwrap();
        println!("{:?}", order);
    }

    #[tokio::test]
    async fn test_query_assigned() {
        let (client, fee_market) = test_fee_market();
        let event = fee_market.contract.abi().event("Assgigned").unwrap();
        let mut filter = FilterBuilder::default();
        filter = filter.from_block(BlockNumber::Earliest);
        filter = filter.address(vec![fee_market.contract.address()]);
        filter = filter.topics(Some(vec![event.signature()]), None, None, None);

        let log = client.eth().logs(filter.build()).await.unwrap();
        for l in log.iter() {
            let raw_log = RawLog {
                topics: l.topics.clone(),
                data: l.data.0.clone(),
            };
            let rl = event.parse_log(raw_log.clone());
            println!("{:?}", l);
            println!("{:?}", rl);
            println!("-----");
        }
    }
}
