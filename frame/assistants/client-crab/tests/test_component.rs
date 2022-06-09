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
        Some(&serde_json::Value::String("Crab".to_string()))
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
async fn test_subscribe() {
    use client_crab::types::runtime_types::{
        bp_header_chain::justification::GrandpaJustification, sp_runtime::generic::header::Header,
    };
    use codec::Decode;
    use futures_util::stream::StreamExt;
    use subxt::sp_runtime::traits::BlakeTwo256;

    let client = common::client().await.unwrap();
    let mut it = client
        .subscribe_grandpa_justifications()
        .await
        .unwrap()
        .take(1);
    if let Some(item) = it.next().await {
        let i =
            GrandpaJustification::<Header<u32, BlakeTwo256>>::decode(&mut item.unwrap().as_ref())
                .unwrap();
        println!("{:?}", i);
    }
}
