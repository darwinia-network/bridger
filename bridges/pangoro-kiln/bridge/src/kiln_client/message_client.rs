use web3::{
    contract::Contract,
    ethabi::{encode, token::Tokenizer},
    transports::Http,
    types::{Address, BlockNumber, Proof as Web3Proof, H256, U256},
    Web3,
};

use crate::message_contract::{inbound::Inbound, outbound::Outbound};

const LANE_IDENTIFY_SLOT: u64 = 0u64;
const LANE_NONCE_SLOT: u64 = 1u64;
const LANE_MESSAGE_SLOT: u64 = 2u64;

pub struct KilnMessageClient {
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
}

impl KilnMessageClient {
    pub fn new(
        endpoint: &str,
        inbound_address: &str,
        outbound_address: &str,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = Web3::new(transport);
        let inbound = Inbound::new(&client, inbound_address)?;
        let outbound = Outbound::new(&client, outbound_address)?;
        Ok(Self {
            client,
            inbound,
            outbound,
        })
    }

    pub async fn get_messages_proof(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<()> {
        let lane_id_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_IDENTIFY_SLOT)],
                block_number,
            )
            .await?;
        let lane_nonce_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_NONCE_SLOT)],
                block_number,
            )
            .await?;
        Ok(())
    }

    pub async fn get_storage_proof(
        &self,
        address: Address,
        storage_keys: Vec<U256>,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<Option<Web3Proof>> {
        Ok(self
            .client
            .eth()
            .proof(address, storage_keys, block_number)
            .await?)
    }
}
