use support_common::error::BridgerError;
use web3::{
    signing::keccak256,
    transports::Http,
    types::{Address, BlockNumber, Bytes, Proof as Web3Proof, U256},
    Web3,
};

use crate::message_contract::{inbound::Inbound, outbound::Outbound};

use super::types::MessagesProof;

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

    pub async fn build_messages_proof(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<MessagesProof> {
        let lane_id_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_IDENTIFY_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get lane_id_proof".into()))?;
        let lane_nonce_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_NONCE_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get lane_nonce_proof".into()))?;
        let message_keys = Self::build_message_storage_keys(begin, end);
        let message_proof = self
            .get_storage_proof(self.outbound.contract.address(), message_keys, block_number)
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get message_proof".into()))?;

        let account_proof = Self::encode_proof(&lane_id_proof.account_proof);
        let lane_id_proof = Self::encode_proof(&lane_id_proof.storage_proof[0].proof);
        let lane_nonce_proof = Self::encode_proof(&lane_nonce_proof.storage_proof[0].proof);
        let lane_messages_proof = message_proof
            .storage_proof
            .iter()
            .map(|x| Self::encode_proof(&x.proof))
            .collect::<Vec<Bytes>>();

        Ok(MessagesProof {
            account_proof,
            lane_id_proof,
            lane_nonce_proof,
            lane_messages_proof,
        })
    }

    fn encode_proof(proofs: &Vec<Bytes>) -> Bytes {
        Bytes::from(
            &rlp::encode_list::<Vec<u8>, _>(
                proofs
                    .iter()
                    .map(|x| x.0.clone())
                    .collect::<Vec<Vec<u8>>>()
                    .as_slice(),
            )[..],
        )
    }

    pub fn build_message_storage_keys(begin: u64, end: u64) -> Vec<U256> {
        (begin..=end)
            .map(|pos| {
                let pos = U256::from(pos);
                let slot = U256::from(LANE_MESSAGE_SLOT);
                let bytes: &mut [u8] = &mut [0u8; 64];
                pos.to_big_endian(&mut bytes[..32]);
                slot.to_big_endian(&mut bytes[32..]);
                U256::from(keccak256(bytes))
            })
            .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> KilnMessageClient {
        KilnMessageClient::new(
            "http://localhost:8545",
            "0xE04c799682F9509CF3D23A15F4A8ddc32648EDd4",
            "0x4214611Be6cA4E337b37e192abF076F715Af4CaE",
        )
        .unwrap()
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_storage_proof() {
        let client = test_client();
        let (begin, end) = (1, 2);
        let message_keys = KilnMessageClient::build_message_storage_keys(begin, end);
        println!("Message keys: {:?}", message_keys);
        let message_proof = client
            .get_storage_proof(client.outbound.contract.address(), message_keys, None)
            .await
            .unwrap()
            .ok_or_else(|| BridgerError::Custom("Failed to get message_proof".into()))
            .unwrap();
        println!("Proof: {:?}", message_proof);
    }
}
