use std::time::Duration;

use web3::{
    api::{Eth, EthFilter, Namespace},
    confirm::wait_for_confirmations,
    types::{H256, U64},
    Transport,
};

async fn transaction_receipt_block_number_check<T: Transport>(
    eth: &Eth<T>,
    hash: H256,
) -> web3::error::Result<Option<U64>> {
    let receipt = eth.transaction_receipt(hash).await?;
    Ok(receipt.and_then(|receipt| receipt.block_number))
}

// Given a transaction hash, wait for confirmations.
async fn wait_for_transaction_confirmation<T: Transport>(
    hash: H256,
    transport: T,
    poll_interval: Duration,
    confirmations: usize,
) -> web3::error::Result<()> {
    let eth = Eth::new(transport.clone());
    if confirmations > 0 {
        let confirmation_check = || transaction_receipt_block_number_check(&eth, hash);
        let eth_filter = EthFilter::new(transport.clone());
        let eth = eth.clone();
        wait_for_confirmations(
            eth,
            eth_filter,
            poll_interval,
            confirmations,
            confirmation_check,
        )
        .await?;
    }
    Ok(())
}
