use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256},
    Web3,
};

use crate::error::BridgeContractResult;

pub struct BeaconLightClient {
    pub contract: Contract<Http>,
}

impl BeaconLightClient {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/BeaconLightClient.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn finalized_header(&self) -> BridgeContractResult<HeaderMessage> {
        let (slot, proposer_index, parent_root, state_root, body_root) = self
            .contract
            .query("finalized_header", (), None, Options::default(), None)
            .await?;
        let header = HeaderMessage {
            slot,
            proposer_index,
            parent_root,
            state_root,
            body_root,
        };
        Ok(header)
    }

    pub async fn sync_committee_roots(&self, period: u64) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query(
                "sync_committee_roots",
                (period,),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn import_finalized_header(
        &self,
        finalized_header_update: FinalizedHeaderUpdate,
        private_key: &SecretKey,
        options: Options,
    ) -> BridgeContractResult<H256> {
        let tx = self
            .contract
            .signed_call(
                "import_finalized_header",
                (finalized_header_update,),
                options,
                private_key,
            )
            .await?;
        Ok(tx)
    }

    pub async fn import_next_sync_committee(
        &self,
        sync_committee_update: SyncCommitteePeriodUpdate,
        private_key: &SecretKey,
        options: Options,
    ) -> BridgeContractResult<H256> {
        let tx = self
            .contract
            .signed_call(
                "import_next_sync_committee",
                (sync_committee_update,),
                options,
                private_key,
            )
            .await?;
        Ok(tx)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, Tokenize},
        ethabi::Token,
        types::{Bytes, H256},
    };

    #[derive(Debug, Clone)]
    pub struct FinalizedHeaderUpdate {
        pub attested_header: HeaderMessage,
        pub signature_sync_committee: SyncCommittee,
        pub finalized_header: HeaderMessage,
        pub finality_branch: Vec<H256>,
        pub sync_aggregate: SyncAggregate,
        pub fork_version: Bytes,
        pub signature_slot: u64,
    }

    #[derive(Debug, Clone)]
    pub struct HeaderMessage {
        pub slot: u64,
        pub proposer_index: u64,
        pub parent_root: H256,
        pub state_root: H256,
        pub body_root: H256,
    }

    #[derive(Debug, Clone)]
    pub struct SyncCommittee {
        pub pubkeys: Vec<Vec<u8>>,
        pub aggregate_pubkey: Bytes,
    }

    #[derive(Debug, Clone)]
    pub struct SyncAggregate {
        pub sync_committee_bits: [H256; 2],
        pub sync_committee_signature: Bytes,
    }

    impl Tokenizable for FinalizedHeaderUpdate {
        fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (
                    attested_header,
                    signature_sync_committee,
                    finalized_header,
                    finality_branch,
                    sync_aggregate,
                    fork_version,
                    signature_slot,
                ) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    attested_header,
                    signature_sync_committee,
                    finalized_header,
                    finality_branch,
                    sync_aggregate,
                    fork_version,
                    signature_slot,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    self.attested_header.clone(),
                    self.signature_sync_committee.clone(),
                    self.finalized_header.clone(),
                    self.finality_branch.clone(),
                    self.sync_aggregate.clone(),
                    Token::FixedBytes(self.fork_version.clone().0),
                    self.signature_slot.clone(),
                )
                    .into_tokens(),
            )
        }
    }

    impl Tokenizable for SyncAggregate {
        fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (sync_committee_bits, sync_committee_signature) =
                    Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    sync_committee_bits,
                    sync_committee_signature,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    Token::FixedArray(
                        self.sync_committee_bits
                            .into_iter()
                            .map(|x| x.into_token())
                            .collect(),
                    ),
                    self.sync_committee_signature.clone(),
                )
                    .into_tokens(),
            )
        }
    }

    impl Tokenizable for SyncCommittee {
        fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (pubkeys, aggregate_pubkey) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    pubkeys,
                    aggregate_pubkey,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    Token::FixedArray(
                        self.pubkeys
                            .iter()
                            .map(|s| Token::Bytes(s.to_vec()))
                            .collect::<Vec<Token>>(),
                    ),
                    self.aggregate_pubkey.0,
                )
                    .into_tokens(),
            )
        }
    }

    impl Tokenizable for HeaderMessage {
        fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (slot, proposer_index, parent_root, state_root, body_root) =
                    Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    slot,
                    proposer_index,
                    parent_root,
                    state_root,
                    body_root,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(vec![
                self.slot.into_token(),
                self.proposer_index.into_token(),
                self.parent_root.into_token(),
                self.state_root.into_token(),
                self.body_root.into_token(),
            ])
        }
    }

    #[derive(Debug, Clone)]
    pub struct SyncCommitteePeriodUpdate {
        pub sync_committee: SyncCommittee,
        pub next_sync_committee_branch: Vec<H256>,
    }

    impl Tokenizable for SyncCommitteePeriodUpdate {
        fn from_token(_token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> Token {
            Token::Tuple(
                (
                    self.sync_committee.into_token(),
                    self.next_sync_committee_branch.into_token(),
                )
                    .into_tokens(),
            )
        }
    }
}
