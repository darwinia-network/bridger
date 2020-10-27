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
type AccountId = <DarwiniaRuntime as System>::AccountId;

/// Sudo Account
pub const ROLE_SUDO: (&str, u8) = ("SUDO", 1);
/// Technical Committee Member
pub const ROLE_TECHNICAL_COMMITTEE: (&str, u8) = ("TECHNICAL_COMMITTEE", 2);
/// Normal Account
pub const ROLE_NORMAL: (&str, u8) = ("NORMAL", 4);

/// contains_role
pub fn contains_role(roles: u8, mask: u8) -> bool {
    roles & mask > 0
}

/// decode_roles
pub fn decode_roles(roles: u8) -> Vec<&'static str> {
    let mut result = vec![];
    if contains_role(roles, ROLE_SUDO.1) {
        result.push(ROLE_SUDO.0);
    }
    if contains_role(roles, ROLE_TECHNICAL_COMMITTEE.1) {
        result.push(ROLE_TECHNICAL_COMMITTEE.0);
    }
    if contains_role(roles, ROLE_NORMAL.1) {
        result.push(ROLE_NORMAL.0);
    }
    result
}

/// encode_roles
pub fn encode_roles(roles: Vec<&'static str>) -> u8 {
    let mut result = 0u8;
    if roles.contains(&ROLE_SUDO.0) {
        result += ROLE_SUDO.1;
    }
    if roles.contains(&ROLE_TECHNICAL_COMMITTEE.0) {
        result += ROLE_TECHNICAL_COMMITTEE.1;
    }
    if roles.contains(&ROLE_NORMAL.0) {
        result += ROLE_NORMAL.1;
    }
    result
}

/// account_roles
pub fn account_roles(account: &AccountId, sudo: &AccountId, tech_comm_members: &[AccountId]) -> u8 {
    let mut roles = vec![];
    roles.push(ROLE_NORMAL.0);
    if sudo == account {
        roles.push(ROLE_SUDO.0);
    }
    if tech_comm_members.contains(&account) {
        roles.push(ROLE_TECHNICAL_COMMITTEE.0);
    }
    encode_roles(roles)
}

/// Account
pub struct Account {
    /// Account Id
    pub account_id: AccountId,
    /// Account Roles
    pub roles: u8,
    /// maybe it is a signer
    pub signer: Option<PairSigner<DarwiniaRuntime, Pair>>,
}

impl Account {
    /// from_seed
    pub fn from_seed(seed: &str, sudo: &AccountId, tech_comm_members: &[AccountId]) -> Account {
        let pair = Pair::from_string(seed, None).unwrap();
        let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);
        let roles = account_roles(&account_id, sudo, tech_comm_members);
        Account {
            account_id,
            roles,
            signer: Some(signer),
        }
    }

    /// from_account_id
    pub fn from_account_id(account_id: AccountId, sudo: &AccountId, tech_comm_members: &[AccountId]) -> Account {
        let roles = account_roles(&account_id, sudo, tech_comm_members);
        Account {
            account_id,
            roles,
            signer: None,
        }
    }

    /// roles
    pub fn role_names(&self) -> Vec<&'static str> {
        decode_roles(self.roles)
    }

    /// is_sudo_key
    pub fn is_sudo_key(&self) -> bool {
        contains_role(self.roles, ROLE_SUDO.1)
    }

    /// is_tech_comm_member
    pub fn is_tech_comm_member(&self) -> bool {
        contains_role(self.roles, ROLE_TECHNICAL_COMMITTEE.1)
    }
}

/// Dawrinia API
pub struct Darwinia {
    client: Client<DarwiniaRuntime>,
    /// Keyring signer
    pub signer: Account,
    /// Proxy real
    pub proxy_real: Option<Account>,
}

impl Darwinia {
    /// New darwinia API
    pub async fn new(config: &Config) -> Result<Darwinia> {
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_url(&config.node)
            .build()
            .await?;
        let sudo = client.key(None).await?;
        let tech_comm_members = client.members(None).await?;

        // build signer
        let signer = Account::from_seed(&config.seed, &sudo, &tech_comm_members);

        // build proxy real
        let real = match &config.proxy {
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
                        let real = AccountId::from(data);
                        Some(real)
                    },
                    Err(_e) => None
                }
            }
        };
        let proxy_real =
            real.map(
                |id| {
                    Account::from_account_id(id, &sudo, &tech_comm_members)
                }
            );

        Ok(Darwinia {
            client,
            signer,
            proxy_real
        })
    }

    /// helper to get signer
    pub fn signer(&self) -> &PairSigner<DarwiniaRuntime, Pair> {
        &self.signer.signer.as_ref().unwrap()
    }

    /// set confirmed with sudo privilege
    pub async fn set_confirmed_parcel(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        let ex = self.client.encode(SetConfirmedParcel {
            ethereum_relay_header_parcel: parcel,
            _runtime: PhantomData::default(),
        })?;
        Ok(self.client.sudo(self.signer(), &ex).await?)
    }

    /// Vote pending relay header parcel
    pub async fn vote_pending_relay_header_parcel(&self, pending: u64, aye: bool) -> Result<H256> {
        if self.signer.is_tech_comm_member() {
            let ex_hash = self.client
                .vote_pending_relay_header_parcel(self.signer(), pending, aye)
                .await?;
            Ok(ex_hash)
        } else {
            Err(Bridger("Not technical committee member".to_string()))
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
                trace!("Proxy call `affirm` for {:?}", real.account_id);
                let affirm = Affirm {
                    _runtime: PhantomData::default(),
                    ethereum_relay_header_parcel: parcel,
                    ethereum_relay_proofs: None,
                };

                let ex = self.client.encode(affirm).unwrap();
                Ok(self.client.proxy(self.signer(), real.account_id.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                Ok(self.client.affirm(self.signer(), parcel, None).await?)
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
                trace!("Proxy call `redeem` for real acount {:?}", real.account_id);
                let redeem = Redeem {
                    _runtime: PhantomData::default(),
                    act: redeem_for,
                    proof,
                };

                let ex = self.client.encode(redeem).unwrap();
                Ok(self.client.proxy(self.signer(), real.account_id.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                Ok(self.client.redeem(self.signer(), redeem_for, proof).await?)
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
