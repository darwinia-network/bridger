use client_contracts::PosaLightClient;
use client_darwinia::client::DarwiniaClient;
use web3::transports::Http;
use web3::Web3;

use relay_e2e::types::ethereum::FastEthereumAccount;
use subquery::Subquery;

#[derive(Clone)]
pub struct EcdsaSource {
    pub block: Option<u32>,
    pub subquery: Subquery,
    pub client_darwinia_web3: Web3<Http>,
    pub client_goerli_web3: Web3<Http>,
    pub client_darwinia_substrate: DarwiniaClient,
    pub client_posa: PosaLightClient,
    pub darwinia_evm_account: FastEthereumAccount,
    pub ethereum_account: FastEthereumAccount,
}
