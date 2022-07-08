use web3::{
    contract::{self, tokens::Detokenize, tokens::Tokenizable},
    types::H256,
};

#[derive(Debug)]
pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: H256,
    pub state_root: H256,
    pub body_root: H256,
}

impl Detokenize for BeaconBlockHeader {
    fn from_tokens(tokens: Vec<web3::ethabi::Token>) -> Result<Self, contract::Error>
    where
        Self: Sized,
    {
        if tokens.len() != 5 {
            Err(contract::Error::InvalidOutputType(String::from(
                "Wrong type!",
            )))
        } else {
            Ok(Self {
                slot: u64::from_token(tokens[0].clone())?,
                proposer_index: u64::from_token(tokens[1].clone())?,
                parent_root: H256::from_token(tokens[2].clone())?,
                state_root: H256::from_token(tokens[3].clone())?,
                body_root: H256::from_token(tokens[4].clone())?,
            })
        }
    }
}
