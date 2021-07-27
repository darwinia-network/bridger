use core::marker::PhantomData;
use std::collections::HashMap;

use substrate_subxt::sp_core::H256;

use support_ethereum::affirmation::RelayVotingState;
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::proxy_type::ProxyType;
use support_ethereum::receipt::{EthereumReceiptProofThing, RedeemFor};

use crate::account::AccountId;
use crate::darwinia::client::Darwinia;
use crate::darwinia::runtime::DarwiniaRuntime;
use crate::error::{Error, Result};
use crate::frame::{
    ethereum::{
        backing::{Redeem, RedeemCallExt},
        game::{AffirmationsStoreExt, EthereumRelayerGame},
        issuing::{RedeemErc20, RedeemErc20CallExt, RegisterErc20, RegisterErc20CallExt},
        relay::{
            Affirm, AffirmCallExt, ConfirmedBlockNumbersStoreExt, EthereumRelay,
            PendingRelayHeaderParcelsStoreExt, SetConfirmedParcel, VotePendingRelayHeaderParcel,
            VotePendingRelayHeaderParcelCallExt,
        },
    },
    proxy::ProxyCallExt,
    sudo::SudoCallExt,
    technical_committee::MembersStoreExt,
};

use super::Account;

type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;
type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<RelayAffirmation>>>;

/// Dawrinia API
#[derive(Clone)]
pub struct Ethereum2Darwinia {
    /// darwinia client
    pub darwinia: Darwinia,
}

impl Ethereum2Darwinia {
    pub fn new(darwinia: Darwinia) -> Self {
        Self { darwinia }
    }

    /// Print Detail
    pub async fn account_detail(&self, block_number: Option<u32>, account: &Account) -> Result<()> {
        log::info!("🧔 ethereum => darwinia account");
        let mut roles = self.darwinia.account_role(&account.0).await?;
        if self.is_tech_comm_member(block_number, account).await? {
            roles.push("TechnicalCommittee".to_string());
        }
        match &account.0.real {
            None => {
                log::info!("🧔 Relayer({:?}): 0x{:?}", roles, &account.0.account_id);
            }
            Some(real_account_id) => {
                log::info!("🧔 Proxy Relayer: 0x{:?}", &account.0.account_id);
                log::info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
            }
        }
        Ok(())
    }

    /// is_tech_comm_member
    pub async fn is_tech_comm_member(
        &self,
        block_number: Option<u32>,
        account: &Account,
    ) -> Result<bool> {
        let block_hash = self.darwinia.block_number2hash(block_number).await?;
        let tech_comm_members = self.darwinia.subxt.members(block_hash).await?;
        Ok(tech_comm_members.contains(account.0.real()))
    }

    /// set confirmed with sudo privilege
    pub async fn set_confirmed_parcel(
        &self,
        account: &Account,
        parcel: EthereumRelayHeaderParcel,
    ) -> Result<H256> {
        let ex = self.darwinia.subxt.encode(SetConfirmedParcel {
            ethereum_relay_header_parcel: parcel,
            _runtime: PhantomData::default(),
        })?;
        Ok(self.darwinia.subxt.sudo(&account.0.signer, &ex).await?)
    }

    /// Vote pending relay header parcel
    pub async fn vote_pending_relay_header_parcel(
        &self,
        account: &Account,
        pending: u64,
        aye: bool,
    ) -> Result<H256> {
        if self.is_tech_comm_member(None, account).await? {
            match &account.0.real {
                Some(real) => {
                    // proxy
                    log::trace!("Proxy vote for {:?}", real);
                    let vote = VotePendingRelayHeaderParcel {
                        block_number: pending,
                        aye,
                        _runtime: PhantomData::default(),
                    };

                    let ex = self.darwinia.subxt.encode(vote).unwrap();
                    let ex_hash = self
                        .darwinia
                        .subxt
                        .proxy(
                            &account.0.signer,
                            real.clone(),
                            Some(ProxyType::EthereumBridge),
                            &ex,
                        )
                        .await?;
                    Ok(ex_hash)
                }
                None => {
                    // no proxy
                    let ex_hash = self
                        .darwinia
                        .subxt
                        .vote_pending_relay_header_parcel(&account.0.signer, pending, aye)
                        .await?;
                    Ok(ex_hash)
                }
            }
        } else {
            Err(Error::NotTechnicalCommitteeMember)
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
        let mut iter = self.darwinia.subxt.affirmations_iter(None).await?;
        while let Some((mut storage_key, affirmations)) = iter.next().await? {
            // get game id
            let game_id: &mut [u8] = &mut storage_key.0[32..40];
            game_id.reverse();
            let game_id =
                u64::from_str_radix(array_bytes::bytes2hex("", game_id).as_str(), 16).unwrap();

            //
            if result.get(&game_id).is_none() {
                result.insert(game_id, HashMap::<u32, Vec<RelayAffirmation>>::new());
            }
            let game = result.get_mut(&game_id).unwrap();

            // get round id
            let round_id: &mut [u8] = &mut storage_key.0[40..44];
            round_id.reverse();
            let round_id =
                u32::from_str_radix(array_bytes::bytes2hex("", round_id).as_str(), 16).unwrap();

            game.insert(round_id, affirmations);
        }
        Ok(result)
    }

    /// affirmations contains block?
    pub fn contains(affirmations: &[RelayAffirmation], block: u64) -> bool {
        for affirmation in affirmations {
            let blocks: &Vec<u64> = &affirmation
                .relay_header_parcels
                .iter()
                .map(|bp| bp.header.number)
                .collect();
            if blocks.contains(&block) {
                return true;
            }
        }

        // TODO: Checking the equality of the affirmations

        // TODO: If there is an affirmation with larger block number, then agree and join in the game.

        // TODO: How to play and join the game
        false
    }

    /// Get confirmed block numbers
    pub async fn confirmed_block_numbers(&self) -> Result<Vec<u64>> {
        Ok(self.darwinia.subxt.confirmed_block_numbers(None).await?)
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
        Ok(self
            .darwinia
            .subxt
            .pending_relay_header_parcels(None)
            .await?)
    }

    /// Submit affirmation
    pub async fn affirm(
        &self,
        account: &Account,
        parcel: EthereumRelayHeaderParcel,
    ) -> Result<H256> {
        match &account.0.real {
            Some(real) => {
                log::trace!("Proxy call `affirm` for {:?}", real);
                let affirm = Affirm {
                    _runtime: PhantomData::default(),
                    ethereum_relay_header_parcel: parcel,
                    ethereum_relay_proofs: None,
                };

                let ex = self.darwinia.subxt.encode(affirm).unwrap();
                Ok(self
                    .darwinia
                    .subxt
                    .proxy(
                        &account.0.signer,
                        real.clone(),
                        Some(ProxyType::EthereumBridge),
                        &ex,
                    )
                    .await?)
            }
            None => Ok(self
                .darwinia
                .subxt
                .affirm(&account.0.signer, parcel, None)
                .await?),
        }
    }

    /// Redeem
    pub async fn redeem(
        &self,
        account: &Account,
        redeem_for: RedeemFor,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        let ethereum_tx_hash = proof
            .header
            .hash
            .map(|hash| array_bytes::bytes2hex("", &hash))
            .ok_or(Error::NoHeaderHashInEthereumReceiptProofOfThing)?;
        match &account.0.real {
            Some(real) => {
                log::trace!(
                    "Proxy redeem ethereum tx 0x{:?} for real account {:?}",
                    ethereum_tx_hash,
                    real
                );
                let redeem = Redeem {
                    _runtime: PhantomData::default(),
                    act: redeem_for,
                    proof,
                };

                let ex = self.darwinia.subxt.encode(redeem).unwrap();
                Ok(self
                    .darwinia
                    .subxt
                    .proxy(
                        &account.0.signer,
                        real.clone(),
                        Some(ProxyType::EthereumBridge),
                        &ex,
                    )
                    .await?)
            }
            None => {
                log::trace!(
                    "Redeem ethereum tx {:?} with account {:?}",
                    ethereum_tx_hash,
                    &account.0.account_id
                );
                Ok(self
                    .darwinia
                    .subxt
                    .redeem(&account.0.signer, redeem_for, proof)
                    .await?)
            }
        }
    }

    /// has_voted
    pub fn has_voted(&self, account: &Account, voting_state: RelayVotingState<AccountId>) -> bool {
        match &account.0.real {
            None => voting_state.contains(&account.0.account_id),
            Some(real) => voting_state.contains(real),
        }
    }

    /// register erc20
    pub async fn register_erc20(
        &self,
        account: &Account,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        match &account.0.real {
            Some(real) => {
                let call = RegisterErc20 {
                    _runtime: PhantomData::default(),
                    proof,
                };

                let ex = self.darwinia.subxt.encode(call).unwrap();
                Ok(self
                    .darwinia
                    .subxt
                    .proxy(
                        &account.0.signer,
                        real.clone(),
                        Some(ProxyType::EthereumBridge),
                        &ex,
                    )
                    .await?)
            }
            None => Ok(self
                .darwinia
                .subxt
                .register_erc20(&account.0.signer, proof)
                .await?),
        }
    }

    /// redeem erc20
    pub async fn redeem_erc20(
        &self,
        account: &Account,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        match &account.0.real {
            Some(real) => {
                let call = RedeemErc20 {
                    _runtime: PhantomData::default(),
                    proof,
                };

                let ex = self.darwinia.subxt.encode(call).unwrap();
                Ok(self
                    .darwinia
                    .subxt
                    .proxy(
                        &account.0.signer,
                        real.clone(),
                        Some(ProxyType::EthereumBridge),
                        &ex,
                    )
                    .await?)
            }
            None => Ok(self
                .darwinia
                .subxt
                .redeem_erc20(&account.0.signer, proof)
                .await?),
        }
    }
}
