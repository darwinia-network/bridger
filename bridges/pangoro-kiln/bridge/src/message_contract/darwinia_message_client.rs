use std::str::FromStr;

use bridge_e2e_traits::strategy::RelayStrategy;

use secp256k1::SecretKey;
use support_common::error::BridgerError;
use web3::{
    transports::Http,
    types::{Address, BlockId, BlockNumber},
    Web3,
};

use client_contracts::{
    inbound_types::ReceiveMessagesProof, outbound_types::ReceiveMessagesDeliveryProof,
    ChainMessageCommitter, Inbound, LaneMessageCommitter,
};
use client_contracts::{FeeMarket, Outbound};

use super::{
    fee_market::FeeMarketRelayStrategy,
    utils::{
        build_darwinia_confirmation_proof, build_darwinia_delivery_proof, build_messages_data,
    },
};

pub struct DarwiniaMessageClient<T: RelayStrategy> {
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
    pub chain_message_committer: ChainMessageCommitter,
    pub lane_message_committer: LaneMessageCommitter,
    pub strategy: T,
    pub private_key: Option<SecretKey>,
}

pub fn build_darwinia_message_client(
    endpoint: &str,
    inbound_address: Address,
    outbound_address: Address,
    chain_message_committer_address: Address,
    lane_message_committer_address: Address,
    fee_market_address: Address,
    account: Address,
    private_key: Option<&str>,
) -> color_eyre::Result<DarwiniaMessageClient<FeeMarketRelayStrategy>> {
    let transport = Http::new(endpoint)?;
    let client = Web3::new(transport);
    let inbound = Inbound::new(&client, inbound_address)?;
    let outbound = Outbound::new(&client, outbound_address)?;
    let fee_market = FeeMarket::new(&client, fee_market_address)?;
    let strategy = FeeMarketRelayStrategy::new(fee_market, account);
    let chain_message_committer =
        ChainMessageCommitter::new(&client, chain_message_committer_address)?;
    let lane_message_committer =
        LaneMessageCommitter::new(&client, lane_message_committer_address)?;
    let private_key = private_key.map(SecretKey::from_str).transpose()?;
    Ok(DarwiniaMessageClient {
        client,
        inbound,
        outbound,
        chain_message_committer,
        lane_message_committer,
        strategy,
        private_key,
    })
}

impl<T: RelayStrategy> DarwiniaMessageClient<T> {
    pub fn private_key(&self) -> color_eyre::Result<SecretKey> {
        Ok(self
            .private_key
            .ok_or_else(|| BridgerError::Custom("Private key not found!".into()))?)
    }

    pub async fn prepare_for_messages_confirmation(
        &self,
        block_id: Option<BlockId>,
    ) -> color_eyre::Result<ReceiveMessagesDeliveryProof> {
        let inbound_lane_data = self.inbound.data().await?;
        let messages_proof = build_darwinia_confirmation_proof(
            &self.inbound,
            &self.lane_message_committer,
            &self.chain_message_committer,
            block_id,
        )
        .await?;
        Ok(ReceiveMessagesDeliveryProof {
            inbound_lane_data,
            messages_proof,
        })
    }

    pub async fn prepare_for_messages_delivery(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<ReceiveMessagesProof> {
        let outbound_lane_data =
            build_messages_data(&self.client, &self.outbound, begin, end).await?;
        let messages_proof = build_darwinia_delivery_proof(
            &self.outbound,
            &self.lane_message_committer,
            &self.chain_message_committer,
            block_number.map(BlockId::from),
        )
        .await?;

        Ok(ReceiveMessagesProof {
            outbound_lane_data,
            messages_proof,
        })
    }
}
