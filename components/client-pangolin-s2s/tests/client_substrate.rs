use sp_core::Pair;

mod common;

#[test]
fn test_account() {
    let signer = "//Alice";
    let pair = sp_core::sr25519::Pair::from_string(&signer, None).unwrap();
    let public = pair.public();
    let account = common_primitives::AccountId::from(public.0);
    assert_eq!(
        account.to_string(),
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    );
}
