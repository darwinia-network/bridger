mod common;

#[tokio::test]
async fn test_client() {
    let _client = common::client().await.unwrap();
}

#[tokio::test]
async fn test_block_hash_from_block_number() {
    let client = common::client().await.unwrap();
    let block_hash = client
        .subxt()
        .rpc()
        .block_hash(Some(1u32.into()))
        .await
        .unwrap();
    assert!(block_hash.is_some());
}

#[tokio::test]
async fn test_spec_version() {
    let client = common::client().await.unwrap();
    let version = client.subxt().rpc().runtime_version(None).await.unwrap();
    assert_eq!(
        version.other.get("specName"),
        Some(&serde_json::Value::String("Pangolin".to_string()))
    );
}

#[tokio::test]
async fn test_system_number() {
    let client = common::client().await.unwrap();
    let number = client
        .runtime()
        .storage()
        .system()
        .number(None)
        .await
        .unwrap();
    println!("{:?}", number);
}

#[tokio::test]
async fn test_technical_committee_members() {
    let client = common::client().await.unwrap();
    let members = client
        .runtime()
        .storage()
        .technical_committee()
        .members(None)
        .await
        .unwrap();
    println!("{:?}", members);
}

#[tokio::test]
async fn test_query_ethereum_relay_confirmed_block_numbers() {
    let client = common::client().await.unwrap();
    let blocks = client
        .runtime()
        .storage()
        .ethereum_relay()
        .confirmed_block_numbers(None)
        .await
        .unwrap();
    println!("{:?}", blocks);
}

// #[tokio::test]
// async fn test_mmr_roots_to_sign() {
//     let client = common::client().await.unwrap();
//     let mmr_roots_to_sign = client
//         .runtime()
//         .storage()
//         .ethereum_relay_authorities()
//         .mmr_roots_to_sign(block_number, exec_block_hash)
//         .await
//         .unwrap();
// }

#[tokio::test]
async fn test_next_term() {
    let client = common::client().await.unwrap();
    let current_term = client
        .ethereum()
        .ethereum_relay_authorities_next_term()
        .await
        .unwrap();
    println!("{}", current_term);
}

#[tokio::test]
async fn authorities_to_sign() {
    let client = common::client().await.unwrap();
    let atd = client
        .runtime()
        .storage()
        .ethereum_relay_authorities()
        .authorities_to_sign(None)
        .await
        .unwrap();
    println!("{:?}", atd);
}

#[tokio::test]
async fn authorities() {
    let client = common::client().await.unwrap();
    let atd = client
        .runtime()
        .storage()
        .ethereum_relay_authorities()
        .authorities(None)
        .await
        .unwrap();
    println!("{:?}", atd);
}
