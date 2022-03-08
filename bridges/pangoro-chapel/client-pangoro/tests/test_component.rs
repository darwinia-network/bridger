use client_pangoro::client::PangoroClient;
use pangoro_subxt::api::runtime_types::bsc_primitives::BscHeader;
use pangoro_subxt::api::runtime_types::ethbloom::Bloom;
use pangoro_subxt::api::runtime_types::primitive_types::{H160, U256};
use subxt::BasicError;
use web3::futures::TryFutureExt;
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber, U64};
use web3::Web3;

mod common;

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

pub async fn get_finalized_checkpoint(client: &PangoroClient) -> Result<BscHeader, BasicError> {
    client
        .runtime()
        .storage()
        .bsc()
        .finalized_checkpoint(None)
        .await
}

#[tokio::test]
async fn test_get_finalized_authority_set() {
    let client = common::client().await.unwrap();
    let result = client
        .runtime()
        .storage()
        .bsc()
        .finalized_authorities(None)
        .await
        .unwrap();
    println!("{:?}", result);
}

#[tokio::test]
async fn test_get_finalized_checkpoint() {
    let client = common::client().await.unwrap();
    let result = get_finalized_checkpoint(&client).await.unwrap();
    println!("{:?}", &result);
}

#[tokio::test]
async fn test_send_finalized_checkpoint() {
    let client = common::client().await.unwrap();
    let web3_client = common::web3_client().unwrap();
    let checkpoint = get_finalized_checkpoint(&client).await.unwrap();
    let mut bsc_headers: Vec<BscHeader> = Vec::new();

    let authorities = client
        .runtime()
        .storage()
        .bsc()
        .finalized_authorities(None)
        .await
        .unwrap();

    for i in 0..(authorities.len() as u64 / 2 + 1) {
        let header = get_bsc_header(&web3_client, checkpoint.number + 200 + i)
            .await
            .unwrap();
        println!("{:?}", header.number);
        bsc_headers.push(header);
    }
    let events = client
        .relay_finalized_epoch_header(bsc_headers)
        .await
        .unwrap();

    // let runtime = client.runtime();
    // let progress = runtime
    //     .tx()
    //     .bsc()
    //     .relay_finalized_epoch_header(bsc_headers)
    //     .sign_and_submit_then_watch(client.account().signer())
    //     .await
    //     .unwrap();
    //
    // let events = progress.wait_for_finalized_success().await.unwrap();
    println!("{:?}", events);
}
