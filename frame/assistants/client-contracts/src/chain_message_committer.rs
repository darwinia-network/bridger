use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, BlockId, H256, U256},
    Web3,
};

use crate::error::BridgeContractResult;

use self::types::{MessageProof, MessageSingleProof};

pub struct ChainMessageCommitter {
    pub contract: Contract<Http>,
}

impl ChainMessageCommitter {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/ChainMessageCommitter.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn commitment(&self) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query("commitment", (), None, Options::default(), None)
            .await?)
    }

    pub async fn proof(
        &self,
        pos: U256,
        at_block: Option<BlockId>,
    ) -> BridgeContractResult<MessageSingleProof> {
        Ok(self
            .contract
            .query("proof", (pos,), None, Options::default(), at_block)
            .await?)
    }

    pub async fn prove(
        &self,
        chain_pos: U256,
        lane_pos: U256,
        block_id: Option<BlockId>,
    ) -> BridgeContractResult<MessageProof> {
        Ok(self
            .contract
            .query(
                "prove",
                (chain_pos, lane_pos),
                None,
                Options::default(),
                block_id,
            )
            .await?)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, TokenizableItem},
        ethabi::Token,
        types::H256,
    };

    #[derive(Debug, Clone)]
    pub struct MessageProof {
        pub chain_proof: MessageSingleProof,
        pub lane_proof: MessageSingleProof,
    }

    impl Tokenizable for MessageProof {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (chain_proof, lane_proof) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    chain_proof,
                    lane_proof,
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
                self.chain_proof.into_token(),
                self.lane_proof.into_token(),
            ])
        }
    }

    #[derive(Debug, Clone)]
    pub struct MessageSingleProof {
        pub root: H256,
        pub proof: Vec<H256>,
    }

    impl TokenizableItem for MessageSingleProof {}
    impl Tokenizable for MessageSingleProof {
        fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (root, proof) = Detokenize::from_tokens(tokens)?;
                Ok(Self { root, proof })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(vec![self.root.into_token(), self.proof.into_token()])
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use web3::types::BlockNumber;

    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::str::FromStr;

    #[tokio::test]
    async fn test_query_commitment() {
        let transport = Http::new("https://pangoro-rpc.darwinia.network").unwrap();
        let client = web3::Web3::new(transport);
        let c = ChainMessageCommitter::new(
            &client,
            Address::from_str("0xbA6c0608f68fA12600382Cd4D964DF9f090AA5B5").unwrap(),
        )
        .unwrap();
        let result = c.commitment().await.unwrap();
        dbg!(result);

        let pos = U256::from_dec_str("1").unwrap();
        let merkle_tree = c.proof(pos, None).await.unwrap();
        println!("{:?}", merkle_tree.root);

        for num in vec![
            149771u64, 148133u64, 148122u64, 148121u64, 148120u64, 148119u64, 148118u64, 148116u64,
            148115u64,
        ] {
            let at_block = BlockId::from(BlockNumber::from(num));
            let merkle_tree = c.proof(pos, Some(at_block)).await.unwrap();
            println!("{:?} {:?}", at_block, merkle_tree.root);
        }
    }
}
