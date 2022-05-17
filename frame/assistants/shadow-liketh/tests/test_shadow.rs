use crate::common::Network;

mod common;

#[tokio::test]
async fn test_mmr_root_with_proof_about_ropsten() {
    let block = 12271421;
    let shadow = common::shadow(Network::Ropsten);
    let parent_mmr_root = shadow.mmr_root(block - 1).await.unwrap();
    let (member_leaf_index, last_leaf_index) = (block - 1, block);
    let mmr_proof = shadow
        .mmr_proof(member_leaf_index, last_leaf_index)
        .await
        .unwrap();
    let parent_mmr_root_hex = array_bytes::bytes2hex("0x", parent_mmr_root);
    println!(
        "PARENT MMR ROOT FOR BLOCK {}: {}",
        block, parent_mmr_root_hex,
    );
    println!(
        "MMR PROOF FOR MEMBER_LEAF_INDEX: {} AND LEAST_LEAF_INDEX: {}",
        member_leaf_index, last_leaf_index,
    );
    for item in mmr_proof {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }
}

#[tokio::test]
async fn test_receipt_about_ropsten() {
    let tx_hash = "0x5c1443fb7d0e478a53e7d96ec05a37ca07be167edf8763fa655e508755e2e1e9";
    let last = 12271421;
    let shadow = common::shadow(Network::Ropsten);
    let receipt = shadow.receipt(tx_hash, last).await.unwrap();
    let mmr_proof = receipt.mmr_proof;
    let receipt_proof_json = serde_json::to_string_pretty(&receipt.receipt).unwrap();
    println!("RECEIPT PROOF JSON: {}", receipt_proof_json);
    println!(
        "MMR PROOF FOR MEMBER_LEAF_INDEX: {} AND LEAST_LEAF_INDEX: {}",
        mmr_proof.member_leaf_index, mmr_proof.last_leaf_index,
    );
    for item in mmr_proof.proof {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }
}
