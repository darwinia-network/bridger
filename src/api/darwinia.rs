//! Darwinia API
use crate::{
    pool::EthereumTransaction,
    result::{Error, Result},
    Config,
};
use primitives::{
    chain::eth::{HeaderStuff, PendingHeader},
    frame::ethereum::{
        backing::VerifiedProofStoreExt,
        game::{PendingHeadersStoreExt, RelayProposalT, RelayProposalsStoreExt},
        relay::{ConfirmedBlockNumbersStoreExt, SubmitProposalCallExt},
    },
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{sp_core::Pair as PairTrait, Client, ClientBuilder, PairSigner};
use web3::types::H256;

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

    /// Get relay proposals
    pub async fn relay_proposals(&self) -> Result<Vec<RelayProposalT>> {
        Ok(self.client.relay_proposals(None).await?)
    }

    /// Get current proposals
    pub async fn current_proposals(&self) -> Result<Vec<u64>> {
        let proposals = self.relay_proposals().await?;
        let mut blocks = vec![];
        for p in proposals {
            blocks.append(
                &mut p
                    .bonded_proposal
                    .iter()
                    .map(|bp| bp.1.header.number)
                    .collect(),
            )
        }

        Ok(blocks)
    }

    /// Get confirmed block numbers
    pub async fn confirmed_block_numbers(&self) -> Result<Vec<u64>> {
        Ok(self.client.confirmed_block_numbers(None).await?)
    }

    /// Get the last confirmed block
    pub async fn last_confirmed(&self) -> Result<u64> {
        Ok(
            if let Some(confirmed) = self.confirmed_block_numbers().await?.iter().max() {
                *confirmed
            } else {
                0
            },
        )
    }

    /// Get pending headers
    pub async fn pending_headers(&self) -> Result<Vec<PendingHeader>> {
        Ok(self.client.pending_headers(None).await?)
    }

    /// Submit Proposal
    pub async fn submit_proposal(&self, proposal: Vec<HeaderStuff>) -> Result<H256> {
        Ok(self.client.submit_proposal(&self.signer, proposal).await?)
    }

    /// Check if should redeem
    pub async fn should_redeem(&self, tx: EthereumTransaction) -> Result<()> {
        if let Some(res) = self
            .client
            .verified_proof(tx.hash(), tx.index, None)
            .await?
        {
            if res {
                Err(Error::Bridger(format!(
                    "The tx {:?} has been redeemed",
                    tx.hash,
                )))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Check if should relay
    pub async fn should_relay(&self, target: u64) -> Result<u64> {
        let last_confirmed = self.last_confirmed().await?;
        if target <= last_confirmed {
            return Err(Error::Bridger(format!(
                "The target block {} is not greater than the last confirmed {}",
                target, last_confirmed,
            )));
        }

        // Check if confirmed
        let confirmed_blocks = self.confirmed_block_numbers().await?;
        if confirmed_blocks.contains(&target) {
            return Err(Error::Bridger(format!(
                "The target block {} has already been submitted",
                target,
            )));
        }

        // Check if the target block is pending
        let pending_headers = self.pending_headers().await?;
        for p in pending_headers {
            if p.1 == target {
                return Err(Error::Bridger(format!(
                    "The target block {} is pending",
                    target,
                )));
            }
        }

        // Check if the target block is in relayer game
        let proposals = self.current_proposals().await?;
        if proposals.contains(&target) {
            return Err(Error::Bridger(format!(
                "The target block {} has been in relayer game",
                target,
            )));
        }
        Ok(last_confirmed)
    }
}
