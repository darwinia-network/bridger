pub use crate::error::BridgeContractResult;
use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

pub struct Outbound {
    pub contract: Contract<Http>,
}

impl Outbound {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract =
            Contract::from_json(client.eth(), address, include_bytes!("abis/Outbound.json"))?;
        Ok(Self { contract })
    }

    pub async fn outbound_lane_nonce(&self) -> BridgeContractResult<OutboundLaneNonce> {
        Ok(self
            .contract
            .query("outboundLaneNonce", (), None, Options::default(), None)
            .await?)
    }

    #[allow(dead_code)]
    pub async fn send_message(
        &self,
        message: SendMessage,
        private_key: &SecretKey,
        fee: U256,
    ) -> BridgeContractResult<H256> {
        let tx = self
            .contract
            .signed_call(
                "send_message",
                message,
                Options {
                    gas: Some(U256::from(10000000u64)),
                    gas_price: Some(U256::from(1300000000u64)),
                    value: Some(fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?;
        Ok(tx)
    }

    pub async fn data(&self) -> BridgeContractResult<OutboundLaneDataStorage> {
        Ok(self
            .contract
            .query("data", (), None, Options::default(), None)
            .await?)
    }

    // Returns (thisChainPosition, thisLanePosition, bridgedChainPosition, bridgedLanePosition)
    pub async fn get_lane_info(&self) -> BridgeContractResult<(u32, u32, u32, u32)> {
        Ok(self
            .contract
            .query("getLaneInfo", (), None, Options::default(), None)
            .await?)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, Tokenize},
        ethabi::{Log, Token},
        types::{Address, Bytes, H256, U256},
    };

    use crate::{error::BridgeContractError, outbound::BridgeContractResult};

    #[derive(Debug)]
    pub struct OutboundLaneNonce {
        pub latest_received_nonce: u64,
        pub latest_generated_nonce: u64,
        pub oldest_unpruned_nonce: u64,
    }

    impl Detokenize for OutboundLaneNonce {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if tokens.len() != 3 {
                return Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from: {:?}",
                    tokens
                )));
            }
            Ok(Self {
                latest_received_nonce: Tokenizable::from_token(tokens[0].clone())?,
                latest_generated_nonce: Tokenizable::from_token(tokens[1].clone())?,
                oldest_unpruned_nonce: Tokenizable::from_token(tokens[2].clone())?,
            })
        }
    }

    #[derive(Debug)]
    pub struct OutboundLaneDataStorage {
        pub latest_received_nonce: u64,
        pub messages: Vec<MessageStorage>,
    }

    #[derive(Debug, Clone)]
    pub struct MessageStorage {
        pub encoded_key: U256,
        pub payload_hash: H256,
    }

    impl Detokenize for MessageStorage {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if tokens.len() != 2 {
                return Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from: {:?}",
                    tokens
                )));
            }

            let (encoded_key, payload_hash): (U256, H256) = Detokenize::from_tokens(tokens)?;
            Ok(Self {
                encoded_key,
                payload_hash,
            })
        }
    }

    impl Detokenize for OutboundLaneDataStorage {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if tokens.len() != 1 {
                return Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from: {:?}",
                    tokens
                )));
            }
            let tokens = match tokens[0].clone() {
                Token::Tuple(tokens) => tokens,
                _ => {
                    return Err(web3::contract::Error::InvalidOutputType(format!(
                        "Failed to decode from: {:?}",
                        tokens
                    )))
                }
            };
            let latest_received_nonce: u64 = Tokenizable::from_token(tokens[0].clone())?;
            if let Token::Array(message_tokens) = tokens[1].clone() {
                let decdoed: Result<Vec<MessageStorage>, web3::contract::Error> = message_tokens
                    .into_iter()
                    .map(|x| {
                        if let Token::Tuple(tokens) = x {
                            Ok(MessageStorage::from_tokens(tokens)?)
                        } else {
                            Err(web3::contract::Error::InvalidOutputType(format!(
                                "Failed to decode from: {:?}",
                                tokens
                            )))
                        }
                    })
                    .collect();
                Ok(Self {
                    latest_received_nonce,
                    messages: decdoed?,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from: {:?}",
                    tokens
                )))
            }
        }
    }

    #[derive(Debug)]
    pub struct SendMessage {
        pub target_contract: Address,
        pub encoded: Bytes,
    }

    impl Tokenize for SendMessage {
        fn into_tokens(self) -> Vec<Token> {
            (self.target_contract, self.encoded).into_tokens()
        }
    }

    #[derive(Debug, Clone)]
    pub struct MessageAccepted {
        pub nonce: u64,
        pub source: Address,
        pub target: Address,
        pub encoded: Bytes,
        pub block_number: u64,
    }

    impl MessageAccepted {
        pub fn from_log(log: Log, block_number: u64) -> BridgeContractResult<Self> {
            fn get_value(log: &Log, name: &str) -> BridgeContractResult<Token> {
                log.params
                    .iter()
                    .find(|&x| x.name == name)
                    .map(|x| x.value.clone())
                    .ok_or_else(|| {
                        BridgeContractError::Custom(format!("Failed to get {:?} from event", name))
                    })
            }

            let nonce = get_value(&log, "nonce")?;
            let source = get_value(&log, "source")?;
            let target = get_value(&log, "target")?;
            let encoded = get_value(&log, "encoded")?;

            Ok(Self {
                nonce: Tokenizable::from_token(nonce)?,
                source: Tokenizable::from_token(source)?,
                target: Tokenizable::from_token(target)?,
                encoded: Tokenizable::from_token(encoded)?,
                block_number,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use web3::contract::Options;
    use web3::ethabi::{RawLog, Token};
    use web3::types::{BlockNumber, FilterBuilder};

    fn test_client() -> (Web3<Http>, Outbound) {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let address = Address::from_str("0xee4f69fc69F2C203a0572e43375f68a6e9027998").unwrap();
        (client.clone(), Outbound::new(&client, address).unwrap())
    }

    #[tokio::test]
    async fn test_outbound() {
        let (client, outbound) = test_client();
        let events: Vec<Token> = outbound
            .contract
            .events("MessageAccepted", (), (), ())
            .await
            .unwrap();
        println!("{:?}", events);

        let event = outbound.contract.abi().event("MessageAccepted").unwrap();
        let mut filter = FilterBuilder::default();
        filter = filter.from_block(BlockNumber::Earliest);
        filter = filter.address(vec![outbound.contract.address()]);
        filter = filter.topics(
            Some(vec![event.signature()]),
            Some(vec![H256::from_low_u64_be(2)]),
            None,
            None,
        );
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
    async fn test_outbound_lane_nonce() {
        let (_, outbound) = test_client();
        let res: (u64, u64, u64) = outbound
            .contract
            .query("outboundLaneNonce", (), None, Options::default(), None)
            .await
            .unwrap();
        println!("nonce: {:?}", res);
    }

    #[tokio::test]
    async fn test_data_() {
        let (_, outbound) = test_client();
        let res = outbound.data().await.unwrap();
        println!("Data: {:?}", res);
    }

    #[tokio::test]
    async fn test_get_lane_info() {
        let (_, outbound) = test_client();
        let res = outbound.get_lane_info().await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_send_message() {
        let (_, outbound) = test_client();
        let private_key = SecretKey::from_str("//Alice").unwrap();
        let send_message = SendMessage {
            target_contract: Address::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            encoded: web3::types::Bytes(vec![]),
        };
        let tx = outbound
            .send_message(send_message, &private_key, U256::from(100000000000000u64))
            .await
            .unwrap();
        println!("Tx: {:?}", tx);
    }
}
