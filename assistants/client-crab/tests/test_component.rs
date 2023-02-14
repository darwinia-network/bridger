use client_crab::types::DarwiniaAccount;
use sp_core::Pair;
use subxt::tx::Signer;
use support_toolkit::convert::SmartCodecMapper;

mod common;

const ALITH: &str = "0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133";
const BALTATHAR: &str = "0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b";

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
    let address = client_crab::runtime_api::storage().system().number();
    let number = client
        .subxt()
        .storage()
        .fetch_or_default(&address, None)
        .await
        .unwrap();
    println!("{:?}", number);
}

#[tokio::test]
async fn test_technical_committee_members() {
    let client = common::client().await.unwrap();
    let address = client_crab::runtime_api::storage()
        .technical_committee()
        .members();
    let members = client
        .subxt()
        .storage()
        .fetch_raw(&address.to_bytes(), None)
        .await
        .unwrap();
    println!("{:?}", members);
}

#[ignore]
#[tokio::test]
async fn test_transfer_call() {
    let client = common::client().await.unwrap();
    let source = DarwiniaAccount::new(ALITH.to_string(), None).unwrap();
    let dest = DarwiniaAccount::new(BALTATHAR.to_string(), None).unwrap();
    let call = client_crab::runtime_api::tx().balances().transfer(
        SmartCodecMapper::map_to(dest.account_id()).unwrap(),
        100 * 10u128.pow(18),
    );
    let tx = client.subxt().tx().call_data(&call).unwrap();
    println!("{:?}", source.signer().address());
    println!("{:?}", array_bytes::bytes2hex("0x", tx));
    let track = client
        .subxt()
        .tx()
        .sign_and_submit_then_watch_default(&call, source.signer())
        .await
        .unwrap();
    let events = track.wait_for_finalized_success().await.unwrap();
    println!("{}", events.extrinsic_index());
    println!("{}", array_bytes::bytes2hex("0x", events.extrinsic_hash()));
}

#[test]
fn test_sign() {
    let pair = ecdsa_pair::crypto::ethereum::Pair::from_string(ALITH, None).unwrap();
    let message = b"a";
    let signature = pair.sign(message);
    println!("{:?}", signature);
}

// #[test]
// fn test_web3_sign() {
//     use web3::signing::{Key, SecretKeyRef};
//
//     let secret_key = &ALITH.replace("0x", "").parse().unwrap();
//     let secret_key_ref = SecretKeyRef::new(&secret_key);
//
//     let message = b"a";
//     // let message = &[0u8; 1];
//     let signature = secret_key_ref
//         .sign_message(&sp_core::keccak_256(message))
//         .unwrap();
//     let v = signature
//         .v
//         .try_into()
//         .expect("signature recovery in electrum notation always fits in a u8");
//
//     let mut bytes = Vec::with_capacity(65);
//     bytes.extend_from_slice(signature.r.as_bytes());
//     bytes.extend_from_slice(signature.s.as_bytes());
//     bytes.push(v);
//     println!("{}", array_bytes::bytes2hex("0x", bytes));
// }

#[tokio::test]
#[cfg(feature = "bridge-s2s")]
async fn test_subscribe() {
    use bridge_s2s_traits::client::S2SClientGeneric;
    use client_crab::types::runtime_types::{
        bp_header_chain::justification::GrandpaJustification, sp_runtime::generic::header::Header,
    };
    use codec::Decode;
    use sp_runtime::traits::BlakeTwo256;

    let client = common::client().await.unwrap();
    let mut it = client.subscribe_grandpa_justifications().await.unwrap();
    if let Some(item) = it.next().await {
        let i =
            GrandpaJustification::<Header<u32, BlakeTwo256>>::decode(&mut item.unwrap().as_ref())
                .unwrap();
        println!("{:?}", i);
    }
}
