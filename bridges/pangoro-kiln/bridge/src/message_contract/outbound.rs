use std::str::FromStr;

pub use types::*;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::Address,
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

    pub async fn send_message(&self) -> color_eyre::Result<()> {
        todo!()
    }

    pub async fn data(&self) -> color_eyre::Result<OutboundLaneDataStorage> {
        Ok(self
            .contract
            .query("data", (), None, Options::default(), None)
            .await?)
    }
}

pub mod types {
    use support_common::error::BridgerError;
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, TokenizableItem},
        ethabi::Token,
        types::{Bytes, U256},
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
            client,
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
}
