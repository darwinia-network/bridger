use crate::{error::Result, Settings};
use darwinia::{
	Darwinia, Darwinia2Ethereum, DarwiniaAccount, Ethereum2Darwinia, FromEthereumAccount,
	ToEthereumAccount,
};
use primitives::{
    frame::{
        ethereum::relay::EthereumRelay,
        bridge::relay_authorities::EthereumRelayAuthorities,
    },
};

use sp_keyring::sr25519::sr25519::{Signature, Public};

use substrate_subxt::{
    sp_runtime::traits::{IdentifyAccount, Verify},
	system::System, SignedExtension, SignedExtra,
    Runtime,
};


/// get darwinia api instance
pub async fn get_darwinia_instance<R: Runtime>(config: &Settings) -> Result<Darwinia<R>> {
	Ok(Darwinia::<R>::new(&config.darwinia.rpc).await?)
}

/// get ethereum to darwinia api instance
pub fn get_e2d_instance<R: Runtime + EthereumRelay>(darwinia: Darwinia<R>) -> Ethereum2Darwinia<R> {
	Ethereum2Darwinia::<R>::new(darwinia)
}

/// get darwinia to ethereum api instance
pub fn get_d2e_instance<R: Runtime>(darwinia: Darwinia<R>) -> Darwinia2Ethereum<R> {
	Darwinia2Ethereum::<R>::new(darwinia)
}

/// get darwinia relayer account
pub fn get_darwinia_account<R: Runtime>(config: &Settings) -> DarwiniaAccount<R> 
where
    <R as Runtime>::Signature: From<Signature>,
    <<R as Runtime>::Signature as Verify>::Signer: From<Public> + IdentifyAccount<AccountId = <R as System>::AccountId>,
    <R as System>::AccountId: Into<<R as System>::Address>,
    <<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send,
{
	DarwiniaAccount::<R>::new(
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
pub fn get_e2d_account<R: Runtime>(account: DarwiniaAccount<R>) -> FromEthereumAccount<R> {
	FromEthereumAccount::<R>::new(account)
}

/// get darwinia to ethereum account
pub fn get_d2e_account<R: Runtime + EthereumRelayAuthorities>(account: DarwiniaAccount<R>, config: &Settings) -> ToEthereumAccount<R> {
	ToEthereumAccount::<R>::new(
		account,
		config.ethereum.authority.clone().map(|a| a.private_key),
		config.ethereum.rpc.to_string(),
	)
}
