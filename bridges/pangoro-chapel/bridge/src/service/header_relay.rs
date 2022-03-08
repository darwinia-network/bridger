use client_pangoro::component::PangoroClientComponent;
use client_pangoro::types::runtime_types::bsc_primitives::BscHeader;
use client_pangoro::types::runtime_types::ethbloom::Bloom;
use client_pangoro::types::runtime_types::primitive_types::{H160, U256};
use lifeline::{Lifeline, Service, Task};
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber, U64};
use web3::Web3;

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{PangoroChapelBus, PangoroChapelConfig};

#[derive(Debug)]
pub struct HeaderRelayService {
    _chapel2pangoro: Lifeline,
}

impl BridgeService for HeaderRelayService {}

impl Service for HeaderRelayService {
    type Bus = PangoroChapelBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        tracing::trace!("Spawn service HeaderRelayService");
        let config: PangoroChapelConfig = Config::restore(Names::BridgePangoroChapel)?;

        let _chapel2pangoro = Self::try_task("header-relay-service", async move {
            tracing::trace!("Start to relay chapel headers to pangoro ");
            start_relay(config).await;
            Ok(())
        });
        Ok(Self { _chapel2pangoro })
    }
}

pub async fn get_bsc_header(
    client: &Web3<Http>,
    block_number: u64,
) -> color_eyre::Result<BscHeader> {
    let block_number = BlockId::Number(BlockNumber::Number(U64::from(block_number)));
    let block = client.eth().block(block_number).await.unwrap().unwrap();

    Ok(BscHeader {
        parent_hash: block.parent_hash,
        uncle_hash: block.uncles_hash,
        coinbase: H160(block.author.0),
        state_root: block.state_root,
        transactions_root: block.transactions_root,
        receipts_root: block.receipts_root,
        log_bloom: Bloom(block.logs_bloom.unwrap().0),
        difficulty: U256(block.difficulty.0),
        number: block.number.unwrap().as_u64(),
        gas_limit: U256(block.gas_limit.0),
        gas_used: U256(block.gas_used.0),
        timestamp: block.timestamp.as_u64(),
        extra_data: block.extra_data.0,
        mix_digest: block.mix_hash.unwrap(),
        nonce: block.nonce.unwrap().0.to_vec(),
    })
}

pub async fn start_relay(config: PangoroChapelConfig) {
    while let Err(e) = relay_chapel_headers(config.clone()).await {
        tracing::error!(target: "pangoro-chapel", "Relaying failed {:?}. Restarting...", e);
    }
}

/// Relay headers to pangoro
/// 1. get finalized checkpoint
/// 2. calculate next bsc header number range which is required by relay interface
/// 3. get current bsc block number.
/// 4. If the current block number is bigger than last required block number,
///    relay the headers. If not, sleep a few seconds, and jump to 3.
pub async fn relay_chapel_headers(config: PangoroChapelConfig) -> color_eyre::Result<()> {
    tracing::trace!(target: "pangoro-chapel", "Initialize Pangoro subxt client");
    let pangoro = PangoroClientComponent::component(config.pangoro).await?;
    let transport = web3::transports::Http::new(&config.chapel.endpoint)?;
    tracing::trace!(target: "pangoro-chapel", "Initialize Chapel web3 client");
    let chapel = web3::Web3::new(transport);

    loop {
        let last_authority_set = pangoro.finalized_authority_set().await?;
        let checkpoint = pangoro.finalized_checkpoint().await?;
        tracing::trace!(target: "pangoro-chapel", "Current checkpoint on Pangoro: {:?}", checkpoint.number);
        tracing::trace!(target: "pangoro-chapel", "Current finalized authority set length on Pangoro: {:?}", last_authority_set.len());

        let next_block_range = 0..(last_authority_set.len() as u64 / 2 + 1);
        let chapel_current_block_number = chapel.eth().block_number().await?.as_u64();
        tracing::trace!(target: "pangoro-chapel", "Current block number on Chapel: {:?}", chapel_current_block_number);

        let mut headers: Vec<BscHeader> = Vec::new();
        if chapel_current_block_number >= next_block_range.end {
            for offset in next_block_range.clone() {
                let block_number = checkpoint.number + offset + 200;
                let header = get_bsc_header(&chapel, block_number).await?;
                headers.push(header);
            }
            tracing::trace!(
                target: "pangoro-chapel",
                "Relaying headers[{:?}:{:?}] to Pangoro",
                checkpoint.number + 200,
                checkpoint.number + 200 + next_block_range.end - 1
            );
            pangoro
                .relay_finalized_epoch_header(headers.clone())
                .await?;
        } else {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}
