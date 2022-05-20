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
    let tx_hash = "0xe9f348a0e02e3be414d692a541240d544c19c3595f6497c349def15e10c86365";
    // let tx_hash = "0x1df2f90123e4ab59cf0c99961c13b5ff4d7b702157bde928b9974068bca7f40a";
    let last = 12277086;
    let shadow = common::shadow(Network::Ropsten);
    let receipt = shadow.receipt(tx_hash, last).await.unwrap();
    let mmr_proof = receipt.mmr_proof;
    let receipt_proof_json = serde_json::to_string_pretty(&receipt.receipt).unwrap();
    println!("RECEIPT PROOF JSON: {}", receipt_proof_json);
    println!(
        "MMR PROOF FOR MEMBER_LEAF_INDEX: {} AND LEAST_LEAF_INDEX: {}",
        mmr_proof.member_leaf_index, mmr_proof.last_leaf_index,
    );
    for item in &mmr_proof.proof {
        let hex = array_bytes::bytes2hex("", item);
        println!("{}", hex);
    }

    let mmr_root = shadow.mmr_root(last - 1).await.unwrap();
    let verified_leaf_position = support_mmr::mmr::leaf_index_to_pos(receipt.receipt.header.number);
    let mmr_size = support_mmr::mmr::leaf_index_to_mmr_size(last - 1);
    let proof = cmmr::MerkleProof::<[u8; 32], MergeHash>::new(mmr_size, mmr_proof.proof);

    let nodes = shadow
        .query_nodes(vec![verified_leaf_position])
        .await
        .unwrap();
    for item in nodes {
        let check_hash = item.hash;
        let result = proof
            .verify(mmr_root, vec![(verified_leaf_position, check_hash)])
            .unwrap();
        println!(
            "check: [{}], {}",
            array_bytes::bytes2hex("", check_hash),
            result,
        );
    }
}

struct MergeHash;

impl cmmr::Merge for MergeHash {
    type Item = [u8; 32];
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        support_mmr::mmr::merge(lhs, rhs)
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

#[test]
fn test_sort_by_refs() {
    let arr0 = [1, 2, 6, 7, 8, 9, 3, 4, 5, 0];
    let mut arr1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    arr1.sort_by(|a, b| {
        arr0.iter()
            .position(|r| r == a)
            .unwrap()
            .cmp(&arr0.iter().position(|r| r == b).unwrap())
    });
    assert_eq!(arr0, arr1);
}
