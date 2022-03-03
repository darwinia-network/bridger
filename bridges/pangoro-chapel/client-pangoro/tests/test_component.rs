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
