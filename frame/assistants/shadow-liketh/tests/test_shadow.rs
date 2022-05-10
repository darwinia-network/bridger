mod common;

#[tokio::test]
async fn test_mmr_root() {
    let shadow = common::shadow();
    let leaf_index = 12244903;
    let data = shadow.mmr_root(leaf_index - 1).await.unwrap();
    let hex = array_bytes::bytes2hex("0x", data);
    println!("{}", hex);
}

#[tokio::test]
async fn test_mmr_proof() {
    let shadow = common::shadow();
    let member_leaf_index = 12244902;
    let last_leaf_index = 12244902;
    let data = shadow
        .mmr_proof(member_leaf_index, last_leaf_index)
        .await
        .unwrap();
    println!("mmr proof size: {}", data.len());
    for item in data {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }
}
