//! Darwinia API
use crate::{result::Result, Config};
use primitives::{
    chain::eth::PendingHeader,
    frame::ethereum::{game::PendingHeadersStoreExt, relay::ConfirmedBlockNumbersStoreExt},
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{sp_core::Pair as PairTrait, Client, ClientBuilder, PairSigner};

/// Dawrinia API
pub struct Darwinia {
    client: Client<DarwiniaRuntime>,
    /// Keyring signer
    pub signer: PairSigner<DarwiniaRuntime, Pair>,
}

impl Darwinia {
    /// New darwinia API
    pub async fn new(config: &Config) -> Result<Darwinia> {
        let pair = Pair::from_string(&config.seed, None).unwrap();
        let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_url(&config.node)
            .build()
            .await?;

        Ok(Darwinia { client, signer })
    }

    /// Get the last confirmed block
    pub async fn last_confirmed(&self) -> Result<Option<u64>> {
        Ok(
            if let Some(confirmed) = self
                .client
                .confirmed_block_numbers(None)
                .await?
                .iter()
                .max()
            {
                Some(*confirmed)
            } else {
                None
            },
        )
    }

    /// Get pending headers
    pub async fn pending_headers(&self) -> Result<Vec<PendingHeader>> {
        Ok(self.client.pending_headers(None).await?)
    }
}
