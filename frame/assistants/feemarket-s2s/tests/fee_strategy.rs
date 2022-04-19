mod common;

use common::TestFeemarketApi;
use feemarket_s2s::fee::{CrazyStrategy, UpdateFeeStrategy};
use relay_pangolin_client::PangolinChain;
use relay_substrate_client::TransactionSignScheme;
use sp_core::Pair;

#[tokio::test]
async fn test_fee_crazy_strategy() {
    let api = TestFeemarketApi;
    let pair =
        <PangolinChain as TransactionSignScheme>::AccountKeyPair::from_string("//Alice", None)
            .unwrap();
    let strategy = CrazyStrategy::<TestFeemarketApi, PangolinChain>::new(api, pair);
    strategy.handle().await.unwrap();
}
