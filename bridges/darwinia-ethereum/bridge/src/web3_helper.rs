use std::cmp;
use std::time::Duration;

use support_etherscan::EtherscanClient;
use web3::{
    api::{Eth, EthFilter, Namespace},
    confirm::wait_for_confirmations,
    transports::Http,
    types::{H256, U256, U64},
    Transport, Web3,
};

async fn transaction_receipt_block_number_check<T: Transport>(
    eth: &Eth<T>,
    hash: H256,
) -> web3::error::Result<Option<U64>> {
    let receipt = eth.transaction_receipt(hash).await?;
    Ok(receipt.and_then(|receipt| receipt.block_number))
}

// Given a transaction hash, wait for confirmations.
pub async fn wait_for_transaction_confirmation<T: Transport>(
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

#[async_trait::async_trait]
pub trait GasPriceOracle {
    fn get_web3(&self) -> &Web3<Http>;
    fn get_etherscan_client(&self) -> Option<&EtherscanClient>;
    fn max_gas_price(&self) -> U256;
    async fn gas_price(&self) -> color_eyre::Result<U256> {
        let price: U256 = match self.get_etherscan_client() {
            Some(etherscan_client) => {
                let oracle = etherscan_client.get_gas_oracle().await?;
                U256::from_dec_str(&oracle.propose_gas_price)? * 1_000_000_000i64
            }
            None => self.get_web3().eth().gas_price().await?,
        };
        Ok(cmp::min(self.max_gas_price(), price))
    }
}
