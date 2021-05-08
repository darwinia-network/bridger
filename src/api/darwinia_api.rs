use crate::{error::Result, Settings};
use darwinia::{
	Darwinia, Darwinia2Ethereum, DarwiniaAccount, Ethereum2Darwinia, FromEthereumAccount,
	ToEthereumAccount,
};
use primitives::runtimes::mainnet::MainnetRuntime;

/// get darwinia api instance
pub async fn get_darwinia_instance(config: &Settings) -> Result<Darwinia<MainnetRuntime>> {
	Ok(Darwinia::new(&config.darwinia.rpc).await?)
}

/// get ethereum to darwinia api instance
pub fn get_e2d_instance(darwinia: Darwinia<MainnetRuntime>) -> Ethereum2Darwinia<MainnetRuntime> {
	Ethereum2Darwinia::new(darwinia)
}

/// get darwinia to ethereum api instance
pub fn get_d2e_instance(darwinia: Darwinia<MainnetRuntime>) -> Darwinia2Ethereum<MainnetRuntime> {
	Darwinia2Ethereum::new(darwinia)
}

/// get darwinia relayer account
pub fn get_darwinia_account(config: &Settings) -> DarwiniaAccount<MainnetRuntime> {
	DarwiniaAccount::new(
		config.darwinia.relayer.private_key.clone(),
		config
			.darwinia
			.relayer
			.real_account
			.clone()
			.map(|real| real[2..].to_string()),
	)
}

/// get ethereum to darwinia account
pub fn get_e2d_account(account: DarwiniaAccount<MainnetRuntime>) -> FromEthereumAccount<MainnetRuntime> {
	FromEthereumAccount::new(account)
}

/// get darwinia to ethereum account
pub fn get_d2e_account(account: DarwiniaAccount<MainnetRuntime>, config: &Settings) -> ToEthereumAccount<MainnetRuntime> {
	ToEthereumAccount::new(
		account,
		config.ethereum.authority.clone().map(|a| a.private_key),
		config.ethereum.rpc.to_string(),
	)
}
