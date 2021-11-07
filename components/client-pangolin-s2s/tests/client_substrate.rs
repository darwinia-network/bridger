use codec::Encode;
use common_primitives::AccountId;
use common_primitives::Balance;
use dp_fee::Relayer;
use sp_core::storage::StorageKey;

mod common;

#[futures_test::test]
async fn test_read_assigned_relayers() {
    let client = common::client().await.unwrap();
    let assigned_relayers: Option<Vec<Relayer<AccountId, Balance>>> = client
        .storage_value(
            StorageKey(
                common::storage_prefix(
                    "FeeMarket".as_bytes(),
                    "AssignedRelayersStorage".as_bytes(),
                )
                .to_vec(),
            ),
            None,
        )
        .await
        .unwrap();
    assert!(assigned_relayers.is_some());
}
