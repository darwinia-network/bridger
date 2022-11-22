use crate::{
    error::BridgeContractResult, fee_market_types::RelayerInfo, simple_fee_market::types::Order,
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
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/SimpleFeeMarket.json"),
        )?;
        Ok(Self { contract })
    }

    #[allow(dead_code)]
    pub async fn enroll(
        &self,
        prev: Address,
        fee: U256,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
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
    pub async fn deposit(&self, fee: U256, private_key: &SecretKey) -> BridgeContractResult<H256> {
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

    pub async fn order(&self, key: U256) -> BridgeContractResult<Order> {
        Ok(self
            .contract
            .query("orderOf", (key,), None, Options::default(), None)
            .await?)
    }

    pub async fn relay_time(&self) -> BridgeContractResult<u64> {
        Ok(self
            .contract
            .query("RELAY_TIME", (), None, Options::default(), None)
            .await?)
    }

    pub async fn get_top_relayer(&self) -> BridgeContractResult<Address> {
        Ok(self
            .contract
            .query("getTopRelayer", (), None, Options::default(), None)
            .await?)
    }

    pub async fn fee_of(&self, relayer_address: Address) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query("feeOf", (relayer_address,), None, Options::default(), None)
            .await?)
    }

    pub async fn balance_of(&self, relayer_address: Address) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query(
                "balanceOf",
                (relayer_address,),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn relayer_info(&self) -> BridgeContractResult<RelayerInfo> {
        let address = self.get_top_relayer().await?;
        let balance = self.balance_of(address).await?;
        let fee = self.fee_of(address).await?;
        Ok(RelayerInfo {
            address,
            balance,
            fee,
        })
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
    use std::str::FromStr;

    use web3::ethabi::{RawLog, Token};
    use web3::types::{BlockNumber, FilterBuilder};

    use super::*;

    fn test_fee_market() -> (Web3<Http>, SimpleFeeMarket) {
        let transport =
            Http::new("https://eth-goerli.g.alchemy.com/v2/WerPq7On62-wy_ARssv291ZPg1TGR5vi")
                .unwrap();
        let client = web3::Web3::new(transport);
        let address = Address::from_str("0x380244554a9C51f0CCaFec90A2766B0C8b698a4a").unwrap();
        let fee_market = SimpleFeeMarket::new(&client, address).unwrap();
        (client, fee_market)
    }

    #[ignore]
    #[tokio::test]
    async fn test_enroll() {
        let (_, fee_market) = test_fee_market();
        let private_key = SecretKey::from_str("").unwrap();
        let tx = fee_market
            .enroll(
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                U256::from(100_000_000_000_000u64),
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
        let private_key = SecretKey::from_str("").unwrap();
        let tx = fee_market
            .deposit(U256::from(10_000_000_000_000_000u64), &private_key)
            .await
            .unwrap();
        println!("{:?}", tx);
    }

    #[ignore]
    #[tokio::test]
    async fn test_query() {
        let (_, fee_market) = test_fee_market();
        let r: Token = fee_market
            .contract
            .query("getTopRelayer", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("{:?}", r);
        let r: Token = fee_market
            .contract
            .query(
                "feeOf",
                (Address::from_str("0x7181932da75bee6d3604f4ae56077b52fb0c5a3b").unwrap(),),
                None,
                Options::default(),
                None,
            )
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
    async fn test_query_relay_time() {
        let (client, fee_market) = test_fee_market();
        let time = fee_market.relay_time().await.unwrap();
        println!("Relay time is : {:?}", time);

        let r = client.eth().gas_price().await.unwrap();
        dbg!(r);
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

    #[tokio::test]
    async fn test_get_top_relayer() {
        let (_, fee_market) = test_fee_market();
        let relayer = fee_market.get_top_relayer().await.unwrap();
        dbg!(&relayer);
        let fee = fee_market.fee_of(relayer).await.unwrap();
        dbg!(&fee);
    }
}
