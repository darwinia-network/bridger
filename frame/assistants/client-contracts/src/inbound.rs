use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

use crate::error::BridgeContractResult;

pub struct Inbound {
    pub contract: Contract<Http>,
}

impl Inbound {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract =
            Contract::from_json(client.eth(), address, include_bytes!("abis/Inbound.json"))?;

        Ok(Self { contract })
    }

    pub async fn inbound_lane_nonce(&self) -> BridgeContractResult<InboundLaneNonce> {
        Ok(self
            .contract
            .query("inboundLaneNonce", (), None, Options::default(), None)
            .await?)
    }

    #[allow(dead_code)]
    pub async fn data(&self) -> BridgeContractResult<InboundLaneData> {
        Ok(self
            .contract
            .query("data", (), None, Options::default(), None)
            .await?)
    }

    pub async fn receive_messages_proof(
        &self,
        messages_proof: ReceiveMessagesProof,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        let tx = self
            .contract
            .signed_call(
                "receive_messages_proof",
                (
                    messages_proof.outbound_lane_data,
                    messages_proof.messages_proof,
                ),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    ..Default::default()
                },
                private_key,
            )
            .await?;
        Ok(tx)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, TokenizableItem},
        ethabi::Token,
        types::{Address, Bytes, U256},
    };

    #[derive(Debug)]
    pub struct InboundLaneNonce {
        pub last_confirmed_nonce: u64,
        pub last_delivered_nonce: u64,
        pub relayer_range_front: u64,
        pub relayer_range_back: u64,
    }

    impl Detokenize for InboundLaneNonce {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            let (
                last_confirmed_nonce,
                last_delivered_nonce,
                relayer_range_front,
                relayer_range_back,
            ): (u64, u64, u64, u64) = Detokenize::from_tokens(tokens)?;
            Ok(Self {
                last_confirmed_nonce,
                last_delivered_nonce,
                relayer_range_front,
                relayer_range_back,
            })
        }
    }

    #[derive(Debug)]
    pub struct InboundLaneData {
        pub relayers: Vec<UnrewardedRelayer>,
        pub last_confirmed_nonce: u64,
        pub last_delivered_nonce: u64,
    }

    impl Detokenize for InboundLaneData {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            match Token::from_tokens(tokens.clone())? {
                Token::Tuple(tokens) if tokens.len() == 3 => {
                    let mut iter = tokens.clone().into_iter();
                    let relayers_token = iter.next().expect("Unreachable!");
                    let relayers = match Token::from_tokens(vec![relayers_token])? {
                        Token::Array(r_tokens) => {
                            let r: Result<Vec<UnrewardedRelayer>, _> = r_tokens
                                .into_iter()
                                .map(|x| UnrewardedRelayer::from_tokens(vec![x]))
                                .collect();
                            r?
                        }
                        _ => {
                            return Err(web3::contract::Error::InvalidOutputType(format!(
                                "Failed to decode from : {:?}",
                                tokens
                            )))
                        }
                    };
                    let last_confirmed_nonce = u64::from_token(iter.next().expect("Unreachable!"))?;
                    let last_delivered_nonce = u64::from_token(iter.next().expect("Unreachable!"))?;
                    Ok(Self {
                        relayers,
                        last_confirmed_nonce,
                        last_delivered_nonce,
                    })
                }
                _ => Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    tokens
                ))),
            }
        }
    }

    #[derive(Debug)]
    pub struct UnrewardedRelayer {
        pub relayer: Address,
        pub delivered_messages: DeliveredMessages,
    }

    impl Detokenize for UnrewardedRelayer {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            match Token::from_tokens(tokens.clone())? {
                Token::Tuple(tokens) if tokens.len() == 2 => {
                    let mut iter = tokens.into_iter();
                    let relayer_token = iter.next().expect("Unreachable!");
                    let relayer = Address::from_token(relayer_token)?;
                    let delivered_messages_token = iter.next().expect("Unreachable!");
                    let delivered_messages =
                        DeliveredMessages::from_tokens(vec![delivered_messages_token])?;
                    Ok(Self {
                        relayer,
                        delivered_messages,
                    })
                }
                _ => Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    tokens
                ))),
            }
        }
    }

    #[derive(Debug)]
    pub struct DeliveredMessages {
        pub begin: u64,
        pub end: u64,
        pub dispatch_results: U256,
    }

    impl Detokenize for DeliveredMessages {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            match Token::from_tokens(tokens.clone())? {
                Token::Tuple(tokens) => {
                    let (begin, end, dispatch_results) = Detokenize::from_tokens(tokens)?;
                    Ok(Self {
                        begin,
                        end,
                        dispatch_results,
                    })
                }
                _ => Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    tokens
                ))),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ReceiveMessagesProof {
        pub outbound_lane_data: OutboundLaneData,
        pub messages_proof: Bytes,
    }

    impl Tokenizable for ReceiveMessagesProof {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (outbound_lane_data, messages_proof) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    outbound_lane_data,
                    messages_proof,
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
                self.outbound_lane_data.into_token(),
                self.messages_proof.into_token(),
            ])
        }
    }

    #[derive(Debug, Clone)]
    pub struct OutboundLaneData {
        pub latest_received_nonce: u64,
        pub messages: Vec<Message>,
    }
    impl Tokenizable for OutboundLaneData {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (latest_received_nonce, messages) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    latest_received_nonce,
                    messages,
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
                self.latest_received_nonce.into_token(),
                self.messages.into_token(),
            ])
        }
    }

    #[derive(Debug, Clone)]
    pub struct Message {
        pub encoded_key: U256,
        pub payload: Payload,
    }

    impl Tokenizable for Message {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (encoded_key, payload) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    encoded_key,
                    payload,
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
                self.encoded_key.into_token(),
                self.payload.into_token(),
            ])
        }
    }
    impl TokenizableItem for Message {}

    #[derive(Debug, Clone)]
    pub struct Payload {
        pub source: Address,
        pub target: Address,
        pub encoded: Bytes,
    }

    impl Tokenizable for Payload {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (source, target, encoded) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    source,
                    target,
                    encoded,
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
                self.source.into_token(),
                self.target.into_token(),
                self.encoded.into_token(),
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn test_client() -> (Web3<Http>, Inbound) {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        (
            client.clone(),
            Inbound::new(
                &client,
                Address::from_str("0x588abe3F7EE935137102C5e2B8042788935f4CB0").unwrap(),
            )
            .unwrap(),
        )
    }

    #[ignore]
    #[tokio::test]
    async fn test_inbound_lane_data() {
        let (_, inbound) = test_client();
        let data = inbound.data().await.unwrap();
        println!("{:?}", data);
    }

    #[ignore]
    #[tokio::test]
    async fn test_inbound_lane_nonce() {
        let (_, inbound) = test_client();
        let nonce = inbound.inbound_lane_nonce().await.unwrap();
        println!("{:?}", nonce);
    }
}
