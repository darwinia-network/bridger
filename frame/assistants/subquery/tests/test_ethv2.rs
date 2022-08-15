use subquery::types::BridgeName;

mod common;

#[tokio::test]
#[cfg(feature = "bridge-ethv2")]
async fn test_collecting_authorities_new_message() {
    let subquery = common::subquery(BridgeName::PangoroGoerli);
    let value = subquery
        .next_collecting_new_message_root_signatures_event(0)
        .await
        .unwrap();
    println!("{:?}", value);
}
