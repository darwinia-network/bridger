mod common;

#[tokio::test]
async fn test_mmr_root() {
    let shadow = common::shadow();
    let data = shadow.mmr_root(12244902).await.unwrap();
    let hex = array_bytes::bytes2hex("", data);
    let r =
        reqwest::get("https://ropsten.shadow.darwinia.network/ethereum/parent_mmr_root/12244903")
            .await
            .unwrap();
    println!("{}", hex);
    println!("{}", r.text().await.unwrap());
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
