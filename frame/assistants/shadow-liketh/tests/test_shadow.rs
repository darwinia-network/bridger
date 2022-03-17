mod common;

#[tokio::test]
async fn test_mmr_root() {
    let shadow = common::shadow();
    let data = shadow.mmr_root(12092405).await.unwrap();
    println!("{:?}", data);
}
