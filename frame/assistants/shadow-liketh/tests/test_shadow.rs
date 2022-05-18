use crate::common::Network;

mod common;

/*
WARNING: Maybe these test will be panic in future
 */

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
        "[ROPSTEN] PARENT MMR ROOT FOR BLOCK {}: {}",
        block, parent_mmr_root_hex,
    );
    println!(
        "[ROPSTEN] MMR PROOF FOR MEMBER_LEAF_INDEX: {} AND LEAST_LEAF_INDEX: {}",
        member_leaf_index, last_leaf_index,
    );
    for item in mmr_proof {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }
}

#[tokio::test]
async fn test_receipt_about_ropsten() {
    let tx_hash = "0x233c17e001ab9ec0440b9fd4229f52e738ea5c5ce47c7676412d40f41b9383aa";
    let last = 12276828;
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

#[tokio::test]
async fn test_mmr_root_with_proof_about_ethereum() {
    let block = 14779883;
    let shadow = common::shadow(Network::Ethereum);
    let parent_mmr_root = shadow.mmr_root(block - 1).await.unwrap();
    let (member_leaf_index, last_leaf_index) = (block - 1, block);
    let mmr_proof = shadow
        .mmr_proof(member_leaf_index, last_leaf_index)
        .await
        .unwrap();
    let parent_mmr_root_hex = array_bytes::bytes2hex("0x", parent_mmr_root);
    println!(
        "[ETHEREUM] PARENT MMR ROOT FOR BLOCK {}: {}",
        block, parent_mmr_root_hex,
    );
    println!(
        "[ETHEREUM] MMR PROOF FOR MEMBER_LEAF_INDEX: {} AND LEAST_LEAF_INDEX: {}",
        member_leaf_index, last_leaf_index,
    );
    for item in mmr_proof {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }
}

#[tokio::test]
async fn test_receipt_about_ethereum() {
    let tx_hash = "0x82125e0829110f9605420671bdb766e4195e37ceef2d250b37961211f1f61ce1";
    let last = 14779883;
    let shadow = common::shadow(Network::Ethereum);
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
