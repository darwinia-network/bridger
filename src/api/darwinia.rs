//! Darwinia API
use crate::{pool::EthereumTransaction, result::Result, Config};
use core::marker::PhantomData;
use primitives::{
    chain::{
        ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
        proxy_type::ProxyType,
    },
    frame::{
        technical_committee::MembersStoreExt,
        ethereum::{
            backing::{RedeemCallExt, VerifiedProofStoreExt, Redeem},
            game::{AffirmationsStoreExt, EthereumRelayerGame},
            relay::{
                AffirmCallExt, ConfirmedBlockNumbersStoreExt,
                SetConfirmedParcel, PendingRelayHeaderParcelsStoreExt,
                Affirm,
                EthereumRelay,
                VotePendingRelayHeaderParcelCallExt,
            },
        },
        sudo::{KeyStoreExt, SudoCallExt},
        proxy::ProxyCallExt,
    },
    runtime::DarwiniaRuntime,
};
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    sp_core::Pair as PairTrait,
    Client, ClientBuilder, PairSigner,
    system::System,
};
use web3::types::H256;
use std::collections::HashMap;
use crate::result::Error::Bridger;

// Types
type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;
type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<RelayAffirmation>>>;

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

    /// Vote pending relay header parcel
    pub async fn vote_pending_relay_header_parcel(&self, pending: u64, aye: bool) -> Result<H256> {
        match self.role {
            Role::TechnicalCommittee => {
                let ex_hash = self.client
                    .vote_pending_relay_header_parcel(&self.signer, pending, aye)
                    .await?;
                Ok(ex_hash)
            }
            _ => Err(Bridger("Not technical committee member".to_string()))
        }
    }


    /// Get all active games' affirmations
    /// games = {
    ///   game_id: {
    ///     round_id: [...]
    ///   }
    /// }
    pub async fn affirmations(&self) -> Result<AffirmationsReturn> {
        let mut result = HashMap::new();
        let mut iter = self.client.affirmations_iter(None).await?;
        while let Some((mut storage_key, affirmations)) = iter.next().await? {
            // get game id
            let game_id: &mut [u8] = &mut storage_key.0[32..40];
            game_id.reverse();
            let game_id = u64::from_str_radix(hex::encode(game_id).as_str(), 16).unwrap();

            //
            if result.get(&game_id).is_none() {
                result.insert(game_id, HashMap::<u32, Vec<RelayAffirmation>>::new());
            }
            let game = result.get_mut(&game_id).unwrap();

            // get round id
            let round_id: &mut [u8] = &mut storage_key.0[40..44];
            round_id.reverse();
            let round_id = u32::from_str_radix(hex::encode(round_id).as_str(), 16).unwrap();

            game.insert(round_id, affirmations);
        }
        Ok(result)
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
    pub async fn pending_headers(&self) -> Result<Vec<PendingRelayHeaderParcel>> {
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

    /// large_block_exists
    pub fn large_block_exists(affirmations: &[RelayAffirmation], block: u64) -> bool {
        for affirmation in affirmations {
            let blocks: &Vec<u64> = &affirmation
                .relay_header_parcels
                .iter()
                .map(|bp| bp.header.number)
                .collect();
            if let Some(max) = blocks.iter().max() {
                if max > &block {
                    return true;
                }
            }
        }
        false
    }

}
