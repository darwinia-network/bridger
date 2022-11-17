use client_contracts::PosaLightClient;
use web3::transports::Http;
use web3::Web3;

use crate::types::ethereum::FastEthereumAccount;
use bridge_e2e_traits::client::EcdsaClient;
use subquery::Subquery;

#[derive(Clone)]
pub struct EcdsaSource<T: EcdsaClient> {
    pub block: Option<u32>,
    pub subquery: Subquery,
    pub client_darwinia_web3: Web3<Http>,
    pub client_eth_web3: Web3<Http>,
    pub client_darwinia_substrate: T,
    pub client_posa: PosaLightClient,
    pub darwinia_evm_account: FastEthereumAccount,
    pub ethereum_account: FastEthereumAccount,
    pub minimal_interval: u64,
}
