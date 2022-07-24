use std::str::FromStr;

use secp256k1::SecretKey;
use support_common::error::BridgerError;
pub use types::*;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

pub struct Outbound {
    pub contract: Contract<Http>,
}

impl Outbound {
    pub fn new(client: &Web3<Http>, address: &str) -> color_eyre::Result<Self> {
        let contract = Contract::from_json(
            client.eth(),
            Address::from_str(address)?,
            include_bytes!("Outbound.json"),
        )?;
        Ok(Self { contract })
    }

    pub async fn outbound_lane_nonce(&self) -> color_eyre::Result<OutboundLaneNonce> {
        Ok(self
            .contract
            .query("outboundLaneNonce", (), None, Options::default(), None)
            .await?)
    }

    pub async fn send_message(
        &self,
        message: SendMessage,
        private_key: SecretKey,
    ) -> color_eyre::Result<H256> {
        let tx = self
            .contract
            .signed_call(
                "send_message",
                message,
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    ..Default::default()
                },
                &private_key,
            )
            .await?;
        Ok(tx)
    }

    pub async fn data(&self) -> color_eyre::Result<OutboundLaneDataStorage> {
        Ok(self
            .contract
            .query("data", (), None, Options::default(), None)
            .await?)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, Tokenize},
        ethabi::Token,
        types::{Address, Bytes, U256},
    };

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

    #[derive(Debug)]
    pub struct MessageStorage {
        pub encoded_key: U256,
        pub payload_hash: Bytes,
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

            let (encoded_key, payload_hash): (U256, Bytes) = Detokenize::from_tokens(tokens)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use web3::contract::Options;
    use web3::ethabi::{RawLog, Token};
    use web3::types::{BlockNumber, FilterBuilder};

    fn test_client() -> (Web3<Http>, Outbound) {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        (
            client.clone(),
            Outbound::new(&client, "0x4214611Be6cA4E337b37e192abF076F715Af4CaE").unwrap(),
        )
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
        filter = filter.topics(Some(vec![event.signature()]), None, None, None);
        let log = client.eth().logs(filter.build()).await.unwrap();
        for l in log.iter() {
            let raw_log = RawLog {
                topics: l.topics.clone(),
                data: l.data.0.clone(),
            };
            let rl = event.parse_log(raw_log);
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
    async fn test_send_message() {
        let (_, outbound) = test_client();
        let private_key = SecretKey::from_str("//Alice").unwrap();
        let send_message = SendMessage {
            target_contract: Address::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            encoded: web3::types::Bytes(vec![]),
        };
        let tx = outbound
            .send_message(send_message, private_key)
            .await
            .unwrap();
        println!("Tx: {:?}", tx);
    }
}
