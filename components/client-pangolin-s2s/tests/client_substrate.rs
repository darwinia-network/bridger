use sp_core::Pair;

mod common;

#[futures_test::test]
async fn test_read_assigned_relayers() {
    let api = common::api().await.unwrap();
    let assigned_relayers = api.assigned_relayers().await.unwrap();
    assert!(!assigned_relayers.is_empty());
}

#[test]
fn test_account() {
    let signer = "//Alice";
    let pair = sp_core::sr25519::Pair::from_string(&signer, None).unwrap();
    let public = pair.public();
    let account = common_primitives::AccountId::from(public.0);
    println!("{:?}", account.to_string());
}
