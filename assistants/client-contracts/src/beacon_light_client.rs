use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    signing::Key,
    transports::Http,
    types::{Address, H256, BlockId, U256},
    Web3,
};

use crate::error::BridgeContractResult;

#[derive(Debug, Clone)]
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

    pub async fn merkle_root(&self, at_block: Option<BlockId>) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query("merkle_root", (), None, Options::default(), at_block)
            .await?)
    }

    pub async fn block_number(&self) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query("block_number", (), None, Options::default(), None)
            .await?)
    }

    pub async fn import_finalized_header(
        &self,
        finalized_header_update: FinalizedHeaderUpdate,
        private_key: &SecretKey,
        mut options: Options,
    ) -> BridgeContractResult<H256> {
        let call = "import_finalized_header";
        let params = (finalized_header_update,).into_tokens();
        let gas = self
            .contract
            .estimate_gas(
                call,
                params.as_slice(),
                private_key.address(),
                Options::default(),
            )
            .await?;
        options.gas = Some(gas);
        let tx = self
            .contract
            .signed_call(call, params.as_slice(), options, private_key)
            .await?;
        Ok(tx)
    }

    pub async fn import_next_sync_committee(
        &self,
        finalized_header_update: FinalizedHeaderUpdate,
        sync_committee_update: SyncCommitteePeriodUpdate,
        private_key: &SecretKey,
        mut options: Options,
    ) -> BridgeContractResult<H256> {
        let call = "import_next_sync_committee";
        let params = (finalized_header_update, sync_committee_update).into_tokens();
        let gas = self
            .contract
            .estimate_gas(
                call,
                params.as_slice(),
                private_key.address(),
                Options::default(),
            )
            .await?;
        options.gas = Some(gas);
        let tx = self
            .contract
            .signed_call(call, params.as_slice(), options, private_key)
            .await?;
        Ok(tx)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable, Tokenize},
        ethabi::Token,
        types::{Address, Bytes, H256, U256},
    };

    #[derive(Debug, Clone)]
    pub struct FinalizedHeaderUpdate {
        pub attested_header: LightClientHeader,
        pub signature_sync_committee: SyncCommittee,
        pub finalized_header: LightClientHeader,
        pub finality_branch: Vec<H256>,
        pub sync_aggregate: SyncAggregate,
        pub fork_version: Bytes,
        pub signature_slot: u64,
    }

    #[derive(Debug, Clone)]
    pub struct LightClientHeader {
        pub beacon: HeaderMessage,
        pub execution: ExecutionPayloadHeader,
        pub execution_branch: Vec<H256>,
    }

    impl Tokenizable for LightClientHeader {
        fn from_token(_token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    self.beacon,
                    self.execution,
                    self.execution_branch,
                )
                    .into_tokens(),
            )
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExecutionPayloadHeader {
        pub parent_hash: H256,
        pub fee_recipient: Address,
        pub state_root: H256,
        pub receipts_root: H256,
        pub logs_bloom: H256,
        pub prev_randao: H256,
        pub block_number: u64,
        pub gas_limit: u64,
        pub gas_used: u64,
        pub timestamp: u64,
        pub extra_data: H256,
        pub base_fee_per_gas: U256,
        pub block_hash: H256,
        pub transactions_root: H256,
        pub withdrawals_root: H256,
    }

    impl Tokenizable for ExecutionPayloadHeader {
        fn from_token(_token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> Token {
            Token::Tuple(
                (
                    self.parent_hash,
                    self.fee_recipient,
                    self.state_root,
                    self.receipts_root,
                    self.logs_bloom,
                    self.prev_randao,
                    self.block_number,
                    self.gas_limit,
                    self.gas_used,
                    self.timestamp,
                    self.extra_data,
                    self.base_fee_per_gas,
                    self.block_hash,
                    self.transactions_root,
                    self.withdrawals_root,
                )
                    .into_tokens(),
            )
        }
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
                    self.attested_header,
                    self.signature_sync_committee,
                    self.finalized_header,
                    self.finality_branch,
                    self.sync_aggregate,
                    Token::FixedBytes(self.fork_version.0),
                    self.signature_slot,
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
                    self.sync_committee_signature,
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
