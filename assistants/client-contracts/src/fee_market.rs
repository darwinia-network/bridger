use crate::error::BridgeContractResult;
use secp256k1::SecretKey;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

use self::types::{Order, OrderExt};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FeeMarket {
    pub contract: Contract<Http>,
}

impl FeeMarket {
    #[allow(dead_code)]
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract =
            Contract::from_json(client.eth(), address, include_bytes!("abis/FeeMarket.json"))?;
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

    pub async fn order(&self, key: U256) -> BridgeContractResult<(Order, Vec<OrderExt>)> {
        Ok(self
            .contract
            .query("getOrder", (key,), None, Options::default(), None)
            .await?)
    }

    pub async fn relay_time(&self) -> BridgeContractResult<u64> {
        Ok(self
            .contract
            .query("RELAY_TIME", (), None, Options::default(), None)
            .await?)
    }

    pub async fn get_relayer(&self, prev: Address) -> BridgeContractResult<Address> {
        Ok(self
            .contract
            .query("relayers", (prev,), None, Options::default(), None)
            .await?)
    }

    pub async fn move_relayer(
        &self,
        old_prev: Address,
        new_prev: Address,
        new_fee: U256,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        let tx = self
            .contract
            .signed_call(
                "move",
                (old_prev, new_prev, new_fee),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    value: Some(new_fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?;
        Ok(tx)
    }
}

pub mod types {
    use web3::contract::tokens::{Detokenize, Tokenizable, TokenizableItem};
    use web3::contract::Error;
    use web3::ethabi::Token;
    use web3::types::{Address, U256};

    #[derive(Debug, Clone)]
    pub struct Order {
        pub assigned_time: u32,
        pub assigned_relayer_number: u32,
        pub collateral: U256,
    }

    #[derive(Debug, Clone)]
    pub struct OrderExt {
        pub assigned_relayer: Address,
        pub maker_fee: U256,
    }

    impl Tokenizable for Order {
        fn from_token(token: Token) -> Result<Self, Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (assigned_time, assigned_relayer_number, collateral) =
                    Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    assigned_time,
                    assigned_relayer_number,
                    collateral,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> Token {
            Token::Tuple(vec![
                self.assigned_time.into_token(),
                self.assigned_relayer_number.into_token(),
                self.collateral.into_token(),
            ])
        }
    }

    impl Tokenizable for OrderExt {
        fn from_token(token: Token) -> Result<Self, Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (assigned_relayer, maker_fee) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    assigned_relayer,
                    maker_fee,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> Token {
            Token::Tuple(vec![
                self.assigned_relayer.into_token(),
                self.maker_fee.into_token(),
            ])
        }
    }
    impl TokenizableItem for OrderExt {}
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use web3::ethabi::{RawLog, Token};
    use web3::signing::Key;
    use web3::types::{BlockNumber, FilterBuilder, Log};

    use super::*;

    fn test_fee_market() -> (Web3<Http>, FeeMarket) {
        // let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let transport = Http::new("https://pangoro-rpc.darwinia.network").unwrap();
        let client = web3::Web3::new(transport);
        let address = Address::from_str("0x6eDcF984eF28C29aa48242B92685244bcD6D7203").unwrap();
        let fee_market = FeeMarket::new(&client, address).unwrap();
        (client, fee_market)
    }

    #[ignore]
    #[tokio::test]
    async fn test_enroll() {
        let (_, fee_market) = test_fee_market();
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
        let privates = vec![
            ("...", 10000000000000000000u64),
            ("...", 10000000000000000000u64),
            ("...", 10000000000000000000u64),
        ];

        for (s, fee) in privates {
            let private_key = SecretKey::from_str(s).unwrap();
            let tx = fee_market
                .deposit(U256::from(fee), &private_key)
                .await
                .unwrap();
            println!("{:?}", tx);
        }
    }

    #[ignore]
    #[tokio::test]
    async fn test_query() {
        let (_, fee_market) = test_fee_market();
        let r: Token = fee_market
            .contract
            .query("getTopRelayers", (), None, Options::default(), None)
            .await
            .unwrap();
        if let Token::Array(tokens) = r {
            for address in tokens {
                let balance: U256 = fee_market
                    .contract
                    .query(
                        "balanceOf",
                        (address.clone(),),
                        None,
                        Options::default(),
                        None,
                    )
                    .await
                    .unwrap();
                let fee: U256 = fee_market
                    .contract
                    .query("feeOf", (address.clone(),), None, Options::default(), None)
                    .await
                    .unwrap();
                println!(
                    "address {:?}; balance {:?}; fee {:?}",
                    address, balance, fee
                );
            }
        }
    }

    #[tokio::test]
    async fn test_query_collateral() {
        let (_, fee_market) = test_fee_market();
        let r: Token = fee_market
            .contract
            .query("collateralPerOrder", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn test_query_relayer_count() {
        let (_, fee_market) = test_fee_market();
        let r: Token = fee_market
            .contract
            .query("relayerCount", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("{:?}", r);
        let r: Token = fee_market
            .contract
            .query("assignedRelayersNumber", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn test_query_order() {
        let (_, fee_market) = test_fee_market();
        let order = fee_market
            .order(U256::from_dec_str("79228162532711081667253501954").unwrap())
            .await
            .unwrap();
        println!("{:?}", order);
    }

    #[tokio::test]
    async fn test_query_relay_time() {
        let (_, fee_market) = test_fee_market();
        let time = fee_market.relay_time().await.unwrap();
        println!("Relay time is : {:?}", time);
    }

    #[tokio::test]
    async fn test_query_assigned() {
        let (client, fee_market) = test_fee_market();
        let event = fee_market.contract.abi().event("Assgigned").unwrap();
        let mut current: u64 = 3628249;
        let interval: u64 = 500;
        let mut logs: Vec<Log> = vec![];
        while logs.is_empty() {
            let mut filter = FilterBuilder::default();
            filter = filter.address(vec![fee_market.contract.address()]);
            filter = filter.topics(Some(vec![event.signature()]), None, None, None);
            filter = filter.to_block(BlockNumber::from(current));
            filter = filter.from_block(BlockNumber::from(current - interval));
            logs = client.eth().logs(filter.build()).await.unwrap();
            dbg!((&current, &logs));
            current -= interval;
        }
        // let log = client.eth().logs(filter.build()).await.unwrap();

        for l in logs.iter() {
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
