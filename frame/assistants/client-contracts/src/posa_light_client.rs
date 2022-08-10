use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{Contract, Options},
    ethabi::Bytes,
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
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/POSALightClient.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn import_message_commitment(
        &self,
        commitment: Commitment,
        signature: Vec<Bytes>,
        private_key: &SecretKey,
        fee: U256,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .signed_call(
                "import_message_commitment",
                (commitment, signature),
                Options {
                    gas: Some(U256::from(10000000u64)),
                    gas_price: Some(U256::from(1300000000u64)),
                    value: Some(fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable},
        ethabi::Token,
        types::{H256, U256},
    };

    #[derive(Debug, Clone)]
    pub struct Commitment {
        pub block_number: u32,
        pub message_root: H256,
        pub nonce: U256,
    }

    impl Tokenizable for Commitment {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (block_number, message_root, nonce) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    block_number,
                    message_root,
                    nonce,
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
                self.block_number.into_token(),
                self.message_root.into_token(),
                self.nonce.into_token(),
            ])
        }
    }
}

#[cfg(test)]
mod tests {}
