use self::types::{BeaconBlockBodyBellatrix, BeaconBlockBodyCapella};
use crate::error::BridgeContractResult;

use secp256k1::SecretKey;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    signing::Key,
    transports::Http,
    types::{Address, BlockId, H256, U256},
    Web3,
};

#[derive(Debug, Clone)]
pub struct ExecutionLayer {
    pub contract: Contract<Http>,
}

impl ExecutionLayer {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/ExecutionLayer.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn merkle_root(&self, at_block: Option<BlockId>) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query("merkle_root", (), None, Options::default(), at_block)
            .await?)
    }

    pub async fn is_capella(&self) -> BridgeContractResult<bool> {
        Ok(self
            .contract
            .query("is_capella", (), None, Options::default(), None)
            .await?)
    }

    pub async fn import_block_body_bellatrix(
        &self,
        beacon_block_body: BeaconBlockBodyBellatrix,
        private_key: &SecretKey,
        mut options: Options,
    ) -> BridgeContractResult<H256> {
        let call = "import_block_body_bellatrix";
        let params = (beacon_block_body,).into_tokens();
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

    pub async fn import_block_body_capella(
        &self,
        beacon_block_body: BeaconBlockBodyCapella,
        private_key: &SecretKey,
        mut options: Options,
    ) -> BridgeContractResult<H256> {
        let call = "import_block_body_capella";
        let params = (beacon_block_body,).into_tokens();
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
        contract::tokens::{Tokenizable, Tokenize},
        ethabi::Token,
        types::{Address, H256, U256},
    };

    #[derive(Debug, Clone)]
    pub struct BeaconBlockBodyBellatrix {
        pub randao_reveal: H256,
        pub eth1_data: H256,
        pub graffiti: H256,
        pub proposer_slashings: H256,
        pub attester_slashings: H256,
        pub attestations: H256,
        pub deposits: H256,
        pub voluntary_exits: H256,
        pub sync_aggregate: H256,
        pub execution_payload: ExecutionPayload,
    }

    impl Tokenizable for BeaconBlockBodyBellatrix {
        fn from_token(_token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> Token {
            Token::Tuple(
                (
                    self.randao_reveal.clone(),
                    self.eth1_data.clone(),
                    self.graffiti.clone(),
                    self.proposer_slashings.clone(),
                    self.attester_slashings.clone(),
                    self.attestations.clone(),
                    self.deposits.clone(),
                    self.voluntary_exits.clone(),
                    self.sync_aggregate.clone(),
                    self.execution_payload.clone(),
                )
                    .into_tokens(),
            )
        }
    }

    #[derive(Debug, Clone)]
    pub struct BeaconBlockBodyCapella {
        pub randao_reveal: H256,
        pub eth1_data: H256,
        pub graffiti: H256,
        pub proposer_slashings: H256,
        pub attester_slashings: H256,
        pub attestations: H256,
        pub deposits: H256,
        pub voluntary_exits: H256,
        pub sync_aggregate: H256,
        pub execution_payload: ExecutionPayloadCapella,
        pub bls_to_execution_changes: H256,
    }

    impl Tokenizable for BeaconBlockBodyCapella {
        fn from_token(_token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> Token {
            Token::Tuple(
                (
                    self.randao_reveal.clone(),
                    self.eth1_data.clone(),
                    self.graffiti.clone(),
                    self.proposer_slashings.clone(),
                    self.attester_slashings.clone(),
                    self.attestations.clone(),
                    self.deposits.clone(),
                    self.voluntary_exits.clone(),
                    self.sync_aggregate.clone(),
                    self.execution_payload.clone(),
                    self.bls_to_execution_changes.clone(),
                )
                    .into_tokens(),
            )
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExecutionPayload {
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
        pub transactions: H256,
    }

    impl Tokenizable for ExecutionPayload {
        fn from_token(_token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    self.parent_hash.clone(),
                    self.fee_recipient.clone(),
                    self.state_root.clone(),
                    self.receipts_root.clone(),
                    self.logs_bloom.clone(),
                    self.prev_randao.clone(),
                    self.block_number,
                    self.gas_limit,
                    self.gas_used,
                    self.timestamp,
                    self.extra_data.clone(),
                    self.base_fee_per_gas.clone(),
                    self.block_hash.clone(),
                    self.transactions.clone(),
                )
                    .into_tokens(),
            )
        }
    }

    #[derive(Debug, Clone)]
    pub struct ExecutionPayloadCapella {
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

    impl Tokenizable for ExecutionPayloadCapella {
        fn from_token(_token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            todo!()
        }

        fn into_token(self) -> web3::ethabi::Token {
            Token::Tuple(
                (
                    self.parent_hash.clone(),
                    self.fee_recipient.clone(),
                    self.state_root.clone(),
                    self.receipts_root.clone(),
                    self.logs_bloom.clone(),
                    self.prev_randao.clone(),
                    self.block_number,
                    self.gas_limit,
                    self.gas_used,
                    self.timestamp,
                    self.extra_data.clone(),
                    self.base_fee_per_gas.clone(),
                    self.block_hash.clone(),
                    self.transactions_root.clone(),
                    self.withdrawals_root.clone(),
                )
                    .into_tokens(),
            )
        }
    }
}
