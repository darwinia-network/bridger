use std::ops::Div;

use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    signing::Key,
    transports::Http,
    types::{Address, BlockId, H256, U256},
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

    pub async fn inbound_lane_nonce(
        &self,
        at_block: Option<BlockId>,
    ) -> BridgeContractResult<InboundLaneNonce> {
        Ok(self
            .contract
            .query("inboundLaneNonce", (), None, Options::default(), at_block)
            .await?)
    }

    #[allow(dead_code)]
    pub async fn data(&self, at_block: Option<BlockId>) -> BridgeContractResult<InboundLaneData> {
        Ok(self
            .contract
            .query("data", (), None, Options::default(), at_block)
            .await?)
    }

    pub async fn receive_messages_proof(
        &self,
        messages_proof: ReceiveMessagesProof,
        delivery_size: U256,
        private_key: &SecretKey,
        mut options: Options,
    ) -> BridgeContractResult<H256> {
        let call = "receive_messages_proof";
        let params = (
            messages_proof.outbound_lane_data,
            messages_proof.messages_proof,
            delivery_size,
        )
            .into_tokens();
        let mut gas = self
            .contract
            .estimate_gas(
                call,
                params.as_slice(),
                private_key.address(),
                Options::default(),
            )
            .await?;
        gas += gas.div(10);
        options.gas = Some(gas);
        let tx = self
            .contract
            .signed_call(call, params.as_slice(), options, private_key)
            .await?;
        Ok(tx)
    }

    // Returns (thisChainPosition, thisLanePosition, bridgedChainPosition, bridgedLanePosition)
    pub async fn get_lane_info(
        &self,
        at_block: Option<BlockId>,
    ) -> BridgeContractResult<(u32, u32, u32, u32)> {
        Ok(self
            .contract
            .query("getLaneInfo", (), None, Options::default(), at_block)
            .await?)
    }
}

pub mod types {
    use crate::error::{BridgeContractResult, BridgeContractError};
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, TokenizableItem},
        ethabi::{Token, Log},
        types::{Address, Bytes, U256},
    };

    #[derive(Default, Debug)]
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

    #[derive(Debug, Clone)]
    pub struct InboundLaneData {
        pub relayers: Vec<UnrewardedRelayer>,
        pub last_confirmed_nonce: u64,
        pub last_delivered_nonce: u64,
    }

    #[derive(Debug, Clone)]
    pub struct UnrewardedRelayer {
        pub relayer: Address,
        pub messages: DeliveredMessages,
    }

    #[derive(Debug, Clone)]
    pub struct DeliveredMessages {
        pub begin: u64,
        pub end: u64,
    }

    impl Tokenizable for DeliveredMessages {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (begin, end) = Detokenize::from_tokens(tokens)?;
                Ok(Self { begin, end })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> Token {
            Token::Tuple(vec![self.begin.into_token(), self.end.into_token()])
        }
    }

    impl Tokenizable for UnrewardedRelayer {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (relayer, messages) = Detokenize::from_tokens(tokens)?;
                Ok(Self { relayer, messages })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> Token {
            Token::Tuple(vec![self.relayer.into_token(), self.messages.into_token()])
        }
    }
    impl TokenizableItem for UnrewardedRelayer {}

    impl Tokenizable for InboundLaneData {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (relayers, last_confirmed_nonce, last_delivered_nonce) =
                    Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    relayers,
                    last_confirmed_nonce,
                    last_delivered_nonce,
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
                self.relayers.into_token(),
                self.last_confirmed_nonce.into_token(),
                self.last_delivered_nonce.into_token(),
            ])
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

    #[derive(Debug, Clone)]
    pub struct MessageDispatched {
        pub nonce: u64,
        pub result: bool,
        pub block_number: u64,
    }

    impl MessageDispatched {
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
            let result = get_value(&log, "result")?;

            Ok(Self {
                nonce: Tokenizable::from_token(nonce)?,
                result: Tokenizable::from_token(result)?,
                block_number,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use web3::{
        ethabi::{RawLog, Token},
        types::{BlockNumber, FilterBuilder},
    };

    use super::*;

    fn test_client() -> (Web3<Http>, Inbound) {
        let transport = Http::new("https://pangoro-rpc.darwinia.network").unwrap();
        // let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        (
            client.clone(),
            Inbound::new(
                &client,
                Address::from_str("0x39539c494dA9b97dD716e167f9cBF25438fe72d0").unwrap(),
                // Address::from_str("0xB0c14Ca271eE4B00ede33505203143C66645f6E4").unwrap(),
            )
            .unwrap(),
        )
    }

    #[ignore]
    #[tokio::test]
    async fn test_inbound_lane_data() {
        let (_, inbound) = test_client();
        let data = inbound.data(None).await.unwrap();
        println!("{:?}", data);
    }

    #[ignore]
    #[tokio::test]
    async fn test_inbound_lane_nonce() {
        let (_, inbound) = test_client();
        let nonce = inbound
            .inbound_lane_nonce(Some(BlockId::Number(BlockNumber::from(1))))
            .await
            .unwrap();
        println!("{:?}", nonce);
    }

    #[ignore]
    #[tokio::test]
    async fn test_query_events() {
        let (client, inbound) = test_client();
        let events: Vec<Token> = inbound
            .contract
            .events("MessageDispatched", (), (), ())
            .await
            .unwrap();
        println!("{:?}", events);

        let event = inbound.contract.abi().event("MessageDispatched").unwrap();
        let mut filter = FilterBuilder::default();
        filter = filter.from_block(BlockNumber::from(3713560u64));
        filter = filter.to_block(BlockNumber::from(3713562u64));
        filter = filter.address(vec![inbound.contract.address()]);
        filter = filter.topics(
            Some(vec![event.signature()]),
            // Some(vec![H256::from_low_u64_be(2)]),
            None,
            None,
            None,
        );
        let log = client.eth().logs(filter.build()).await.unwrap();
        dbg!(&log);
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
