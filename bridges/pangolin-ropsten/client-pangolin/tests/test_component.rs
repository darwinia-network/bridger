mod common;

#[tokio::test]
async fn test_client() {
    let _client = common::client().await.unwrap();
}
