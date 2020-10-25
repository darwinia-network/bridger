//! Darwinia API
use crate::{pool::EthereumTransaction, result::Result, Config};
use core::marker::PhantomData;
use primitives::{
    chain::{
        ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
        proxy_type::ProxyType,
    },
    frame::{
        collective::{ExecuteCallExt, MembersStoreExt},
        ethereum::{
            backing::{RedeemCallExt, VerifiedProofStoreExt, Redeem},
            game::{AffirmationsStoreExt, EthereumRelayerGame, PendingRelayHeaderParcelsStoreExt},
            relay::{
                AffirmCallExt, ApprovePendingRelayHeaderParcel, ConfirmedBlockNumbersStoreExt,
                RejectPendingRelayHeaderParcel, SetConfirmedParcel,
                Affirm,
            },
        },
        sudo::{KeyStoreExt, SudoCallExt},
        proxy::ProxyCallExt,
    },
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    sp_core::{Encode, Pair as PairTrait},
    Client, ClientBuilder, PairSigner,
    system::System,
};
use web3::types::H256;

// Types
type PendingHeader = <DarwiniaRuntime as EthereumRelayerGame>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;

/// Account Role
#[derive(PartialEq, Eq, Debug)]
pub enum Role {
    /// Sudo Account
    Sudo,
    /// Technical Committee Member
    TechnicalCommittee,
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
    /// Proxy real
    pub proxy_real: Option<<DarwiniaRuntime as System>::AccountId>,
}

type NotRelayReason = String;

impl Darwinia {
    /// New darwinia API
    pub async fn new(config: &Config) -> Result<Darwinia> {
        let pair = Pair::from_string(&config.seed, None).unwrap();
        let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);

        // proxy
        let proxy_real = match &config.proxy {
            None => None,
            Some(proxy) => {

                // get real account str
                let real = if &proxy.real[0..2] == "0x" {
                    &proxy.real[2..]
                } else {
                    &proxy.real
                };

                // convert to account id
                match hex::decode(real) {
                    Ok(real) => {
                        let mut data: [u8; 32] = [0u8; 32];
                        data.copy_from_slice(&real[..]);
                        let real = <DarwiniaRuntime as System>::AccountId::from(data);
                        Some(real)
                    },
                    Err(_e) => None
                }
            }
        };

        // client
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_url(&config.node)
            .build()
            .await?;

        let pk = signer.signer().public().to_string();
        let sudo = client.key(None).await?.to_string();
        let technical_committee_members = client.members(None).await?;

        Ok(Darwinia {
            client,
            signer,
            role: if sudo == pk {
                Role::Sudo
            } else if technical_committee_members.iter().any(|cpk| cpk.to_string() == pk) {
                Role::TechnicalCommittee
            } else {
                Role::Normal
            },
            proxy_real
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
            Role::TechnicalCommittee => {
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
            Role::TechnicalCommittee => {
                self.client
                    .execute(&self.signer, &ex, ex.size_hint() as u32)
                    .await?
            }
            Role::Normal => H256::from([0; 32]),
        })
    }

    /// Get all active games' affirmations
    pub async fn affirmations(&self) -> Result<Vec<RelayAffirmation>> {
        let mut affirmations = vec![];
        let mut iter = self.client.affirmations_iter(None).await?;
        if let Some((_, mut p)) = iter.next().await? {
            affirmations.append(&mut p);
        }
        Ok(affirmations)
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

    /// Submit affirmation
    pub async fn affirm(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        match &self.proxy_real {
            Some(real) => {
                trace!("Proxy call `affirm` for {:?}", real);
                let affirm = Affirm {
                    _runtime: PhantomData::default(),
                    ethereum_relay_header_parcel: parcel,
                    ethereum_relay_proofs: None,
                };

                let ex = self.client.encode(affirm).unwrap();
                Ok(self.client.proxy(&self.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                Ok(self.client.affirm(&self.signer, parcel, None).await?)
            }
        }
    }

    /// Redeem
    pub async fn redeem(
        &self,
        redeem_for: RedeemFor,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        match &self.proxy_real {
            Some(real) => {
                trace!("Proxy call `redeem` for real acount {:?}", real);
                let redeem = Redeem {
                    _runtime: PhantomData::default(),
                    act: redeem_for,
                    proof,
                };

                let ex = self.client.encode(redeem).unwrap();
                Ok(self.client.proxy(&self.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                Ok(self.client.redeem(&self.signer, redeem_for, proof).await?)
            }
        }
    }

    /// Check if should redeem
    pub async fn should_redeem(&self, tx: &EthereumTransaction) -> Result<bool> {
        Ok(!self
            .client
            .verified_proof((tx.block_hash.to_fixed_bytes(), tx.index), None)
            .await?
            .unwrap_or(false))
    }


    /// Check if should relay
    pub async fn should_relay(&self, target: u64) -> Result<Option<NotRelayReason>> {
        let last_confirmed = self.last_confirmed().await?;

        if target <= last_confirmed {
            let reason =
                format!(
                    "The target block {} is less than the last_confirmed {}",
                    &target,
                    &last_confirmed
                );
            trace!("{}", &reason);
            return Ok(Some(reason));
        }

        // Check if confirmed
        let confirmed_blocks = self.confirmed_block_numbers().await?;
        if confirmed_blocks.contains(&target) {
            let reason = format!("The target block {} has been confirmed", &target);
            trace!("{}", &reason);
            return Ok(Some(reason));
        }

        // Check if the target block is pending
        let pending_headers = self.pending_headers().await?;
        for p in pending_headers {
            if p.1 == target {
                let reason = format!("The target block {} is pending", &target);
                trace!("{}", &reason);
                return Ok(Some(reason));
            }
        }

        // Check if the target block is in affirmations
        for affirmation in self.affirmations().await? {
            let blocks_of_affirmation: &Vec<u64> =
                &affirmation.relay_header_parcels.iter().map(|bp| bp.header.number).collect();
            if blocks_of_affirmation.contains(&target) {
                let reason = format!("The target block {} is in the relayer game", &target);
                trace!("{}", &reason);
                return Ok(Some(reason));
            }
        }

        Ok(None)
    }
}
