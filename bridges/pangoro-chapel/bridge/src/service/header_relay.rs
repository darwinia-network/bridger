use lifeline::{Lifeline, Service, Task};
use support_common::error::BridgerError;
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber, U256, U64};
use web3::Web3;

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{PangoroChapelBus, PangoroChapelConfig};
use crate::pangoro_client::client::PangoroClient;
use crate::pangoro_client::types::BSCHeader;

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
) -> color_eyre::Result<BSCHeader> {
    let block_number = BlockId::Number(BlockNumber::Number(U64::from(block_number)));
    let block = client
        .eth()
        .block(block_number)
        .await?
        .ok_or_else(|| BridgerError::Custom("Failed to get block".into()))?;

    let log_bloom = web3::types::Bytes(
        block
            .logs_bloom
            .ok_or_else(|| BridgerError::Custom("Failed to get log_bloom from block".into()))?
            .0
            .into(),
    );
    let number = U256::from(
        block
            .number
            .ok_or_else(|| BridgerError::Custom("Failed to get number from block".into()))?
            .as_u64(),
    );
    let nonce = block
        .nonce
        .ok_or_else(|| BridgerError::Custom("Failed to get nonce from block".into()))?
        .0;
    let mix_digest = block
        .mix_hash
        .ok_or_else(|| BridgerError::Custom("Failed to get mix_digest from block".into()))?;

    Ok(BSCHeader {
        parent_hash: block.parent_hash,
        uncle_hash: block.uncles_hash,
        coinbase: block.author,
        state_root: block.state_root,
        transactions_root: block.transactions_root,
        receipts_root: block.receipts_root,
        difficulty: block.difficulty,
        gas_limit: block.gas_limit,
        gas_used: block.gas_used,
        timestamp: block.timestamp,
        extra_data: block.extra_data,
        mix_digest,
        number,
        log_bloom,
        nonce,
    })
}

pub async fn start_relay(config: PangoroChapelConfig) {
    while let Err(e) = relay_chapel_headers(config.clone()).await {
        tracing::error!(target: "pangoro-chapel", "Relaying failed {:?}. Restarting...", e);
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
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
    let pangoro = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.bsc_address,
        Some(&config.pangoro.private_key),
    )?;
    let transport = web3::transports::Http::new(&config.chapel.endpoint)?;
    tracing::trace!(target: "pangoro-chapel", "Initialize Chapel web3 client");
    let chapel = web3::Web3::new(transport);

    loop {
        let authority_set_length = pangoro.get_authority_set_length().await?;
        let checkpoint = pangoro.get_finalized_checkpoint().await?;
        let checkpoint_number = checkpoint.4.as_u64();
        tracing::trace!(target: "pangoro-chapel", "Current checkpoint on Pangoro: {:?}", checkpoint_number);
        tracing::trace!(target: "pangoro-chapel", "Current finalized authority set length on Pangoro: {:?}", authority_set_length);

        let next_block_range = 0..(authority_set_length.as_u64() / 2 + 1);
        let chapel_current_block_number = chapel.eth().block_number().await?.as_u64();
        tracing::trace!(target: "pangoro-chapel", "Current block number on Chapel: {:?}", chapel_current_block_number);

        let mut headers: Vec<BSCHeader> = Vec::new();
        if chapel_current_block_number >= checkpoint_number + next_block_range.end {
            for offset in next_block_range.clone() {
                let block_number = checkpoint_number + offset + 200;
                let header = get_bsc_header(&chapel, block_number).await?;
                headers.push(header);
            }
            tracing::trace!(
                target: "pangoro-chapel",
                "Relaying headers[{:?}:{:?}] to Pangoro",
                checkpoint_number + 200,
                checkpoint_number + 200 + next_block_range.end - 1
            );
            let tx = pangoro.import_finalized_epoch_header(headers).await?;
            tracing::info!(
                target: "pangoro-chapel",
                "Sending tx: {:?}",
                tx
            );
        }
        tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    }
}
