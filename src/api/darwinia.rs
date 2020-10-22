//! Darwinia API
use crate::{pool::EthereumTransaction, result::Result, Config};
use core::marker::PhantomData;
use primitives::{
    chain::ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
    frame::{
        collective::{ExecuteCallExt, MembersStoreExt},
        ethereum::{
            backing::{RedeemCallExt, VerifiedProofStoreExt},
            game::{AffirmationsStoreExt, EthereumRelayerGame, PendingRelayHeaderParcelsStoreExt},
            relay::{
                AffirmCallExt, ApprovePendingRelayHeaderParcel, ConfirmedBlockNumbersStoreExt,
                RejectPendingRelayHeaderParcel, SetConfirmedParcel,
            },
        },
        sudo::{KeyStoreExt, SudoCallExt},
    },
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    sp_core::{Encode, Pair as PairTrait},
    Client, ClientBuilder, PairSigner,
};
use web3::types::H256;

// Types
type PendingHeader = <DarwiniaRuntime as EthereumRelayerGame>::PendingRelayHeaderParcel;
type RelayProposal = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;

/// Account Role
#[derive(PartialEq, Eq)]
pub enum Role {
    /// Sudo Account
    Sudo,
    /// Council Member
    Council,
    /// Normal Account
    Normal,
}

/// Dawrinia API
pub struct Darwinia {
    client: Client<DarwiniaRuntime>,
    /// Keyring signer
    pub signer: PairSigner<DarwiniaRuntime, Pair>,
    /// Account Role
    pub role: Role,
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

        let pk = signer.signer().public().to_string();
        let sudo = client.key(None).await?.to_string();
        let council = client.members(None).await?;

        Ok(Darwinia {
            client,
            signer,
            role: if sudo == pk {
                Role::Sudo
            } else if council.iter().any(|cpk| cpk.to_string() == pk) {
                Role::Council
            } else {
                Role::Normal
            },
        })
    }

    /// set confirmed with sudo privilege
    pub async fn set_confirmed_parcel(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        let ex = self.client.encode(SetConfirmedParcel {
            ethereum_relay_header_parcel: parcel,
            _runtime: PhantomData::default(),
        })?;
        Ok(self.client.sudo(&self.signer, &ex).await?)
    }

    /// Approve pending header
    pub async fn approve_pending_header(&self, pending: u64) -> Result<H256> {
        let ex = self.client.encode(ApprovePendingRelayHeaderParcel {
            pending,
            _runtime: PhantomData::default(),
        })?;
        Ok(match self.role {
            Role::Sudo => self.client.sudo(&self.signer, &ex).await?,
            Role::Council => {
                self.client
                    .execute(&self.signer, &ex, ex.size_hint() as u32)
                    .await?
            }
            Role::Normal => H256::from([0; 32]),
        })
    }

    /// Reject pending header
    pub async fn reject_pending_header(&self, pending: u64) -> Result<H256> {
        let ex = self.client.encode(RejectPendingRelayHeaderParcel {
            pending,
            _runtime: PhantomData::default(),
        })?;
        Ok(match self.role {
            Role::Sudo => self.client.sudo(&self.signer, &ex).await?,
            Role::Council => {
                self.client
                    .execute(&self.signer, &ex, ex.size_hint() as u32)
                    .await?
            }
            Role::Normal => H256::from([0; 32]),
        })
    }

    /// Get relay proposals
    pub async fn proposals(&self) -> Result<Vec<RelayProposal>> {
        let mut proposals = vec![];
        let mut iter = self.client.affirmations_iter(None).await?;
        if let Some((_, mut p)) = iter.next().await? {
            proposals.append(&mut p);
        }
        Ok(proposals)
    }

    /// Get current proposals
    pub async fn current_proposals(&self) -> Result<Vec<u64>> {
        let proposals = self.proposals().await?;
        let mut blocks = vec![];
        for p in proposals {
            blocks.append(
                &mut p
                    .relay_header_parcels
                    .iter()
                    .map(|bp| bp.header.number)
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
        Ok(self.client.pending_relay_header_parcels(None).await?)
    }

    /// Submit Proposal
    pub async fn affirm(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        Ok(self.client.affirm(&self.signer, parcel, None).await?)
    }

    /// Redeem
    pub async fn redeem(
        &self,
        redeem_for: RedeemFor,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        Ok(self.client.redeem(&self.signer, redeem_for, proof).await?)
    }

    /// Check if should redeem
    pub async fn should_redeem(&self, tx: &EthereumTransaction) -> Result<bool> {
        Ok(!self
            .client
            .verified_proof((tx.hash(), tx.index), None)
            .await?
            .unwrap_or(false))
    }

    /// Check if should relay
    pub async fn should_relay(&self, target: u64) -> Result<Option<u64>> {
        let last_confirmed = self.last_confirmed().await?;
        if target <= last_confirmed {
            trace!(
                "The target block {} is less than the last_confirmed",
                &last_confirmed
            );
            return Ok(None);
        }

        // Check if confirmed
        let confirmed_blocks = self.confirmed_block_numbers().await?;
        if confirmed_blocks.contains(&target) {
            trace!("The target block {} has been confirmed", &target);
            return Ok(None);
        }

        // Check if the target block is pending
        let pending_headers = self.pending_headers().await?;
        for p in pending_headers {
            if p.1 == target {
                trace!("The target block {} is pending", &target);
                return Ok(None);
            }
        }

        // Check if the target block is in relayer game
        let proposals = self.current_proposals().await?;
        if !proposals.is_empty() && proposals.contains(&target) {
            trace!("The target block {} is in the relayer game", &target);
            return Ok(None);
        }

        Ok(Some(last_confirmed))
    }
}
