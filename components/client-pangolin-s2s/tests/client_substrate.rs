use codec::Encode;
use common_primitives::AccountId;
use common_primitives::Balance;
use dp_fee::Relayer;

mod common;

#[futures_test::test]
async fn test_read_assigned_relayers() {
    let client = common::client().await.unwrap();
    let assigned_relayers: Option<Vec<Relayer<AccountId, Balance>>> = client
        .storage_value(
            bp_runtime::storage_map_final_key_blake2_128concat(
                "feeMarket",
                "assignedRelayersStorage",
                &[],
            ),
            None,
        )
        .await
        .unwrap();
    println!("{:?}", assigned_relayers);
}
