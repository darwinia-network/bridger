use crate::api::darwinia::AccountId;
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::sp_core::ecdsa::Pair as EcdsaPair;
use substrate_subxt::{Client, sp_core::Pair as PairTrait, PairSigner};
use primitives::runtime::DarwiniaRuntime;
use crate::error::Result;
use primitives::{
    chain::RelayVotingState,
    frame::{
        sudo::KeyStoreExt,
        technical_committee::MembersStoreExt,
        bridge::relay_authorities::AuthoritiesStoreExt
    }
};

#[derive(Debug, PartialEq)]
enum Role {
    Normal,
    TechnicalCommittee,
    Sudo,
    Authority,
}

fn build_roles(account: &AccountId, sudo: &AccountId, tech_comm_members: &[AccountId], authorities: &[AccountId]) -> Vec<Role> {
    let mut roles: Vec<Role> = vec![];
    roles.push(Role::Normal);
    if sudo == account {
        roles.push(Role::Sudo);
    }
    if tech_comm_members.contains(&account) {
        roles.push(Role::TechnicalCommittee);
    }
    if authorities.contains(&account) {
        roles.push(Role::Authority);
    }

    roles
}

/// Account
pub struct DarwiniaSender {
    /// client
    pub client: Client<DarwiniaRuntime>,
    /// Account Id
    pub account_id: AccountId,
    /// signer of the account
    pub signer: PairSigner<DarwiniaRuntime, Pair>,
    /// ecdsa pair of account
    pub ecdsa_pair: EcdsaPair,
    /// proxy real
    pub real: Option<AccountId>,
}

impl DarwiniaSender {
    /// Create a new Account
    pub fn new(seed: String, real: Option<String>, client: Client<DarwiniaRuntime>) -> DarwiniaSender {
        let ecdsa_pair = EcdsaPair::from_string(&seed, None).unwrap();

        //
        let pair = Pair::from_string(&seed, None).unwrap(); // if not a valid seed
        let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);

        // convert to account id
        let real = real.map(|real| {
            let real = hex::decode(real).unwrap(); // if decode fail
            let mut data: [u8; 32] = [0u8; 32];
            data.copy_from_slice(&real[..]);
            AccountId::from(data)
        });

        DarwiniaSender {
            client,
            account_id,
            signer,
            ecdsa_pair,
            real,
        }
    }

    async fn roles(&self) -> Result<Vec<Role>> {
        let sudo = self.client.key(None).await?;
        let tech_comm_members = self.client.members(None).await?;
        let authorities = self.client.authorities(None).await?
            .iter()
            .map(|a| a.account_id.clone() )
            .collect::<Vec<_>>();

        let roles = if let Some(real_account_id) = &self.real {
            build_roles(real_account_id, &sudo, &tech_comm_members, &authorities)
        } else {
            build_roles(&self.account_id, &sudo, &tech_comm_members, &authorities)
        };

        Ok(roles)
    }

    /// role names
    pub async fn role_names(&self) -> Result<Vec<String>> {
        let roles = self.roles().await?
            .iter()
            .map(|role| format!("{:?}", role))
            .collect::<Vec<String>>();
        Ok(roles)
    }

    /// is_sudo_key
    pub async fn is_sudo_key(&self) -> Result<bool> {
        let roles = self.roles().await?;
        Ok(roles.contains(&Role::Sudo))
    }

    /// is_tech_comm_member
    pub async fn is_tech_comm_member(&self) -> Result<bool> {
        let roles = self.roles().await?;
        Ok(roles.contains(&Role::TechnicalCommittee))
    }

    /// is_authority
    pub async fn is_authority(&self) -> Result<bool> {
        let roles = self.roles().await?;
        Ok(roles.contains(&Role::Authority))
    }

    /// has_voted
    pub fn has_voted(&self, voting_state: RelayVotingState<AccountId>) -> bool {
        match &self.real {
            None => voting_state.contains(&self.account_id),
            Some(real) => voting_state.contains(real)
        }
    }

    /// sign
    pub fn ecdsa_sign(&self, message: &[u8]) -> Result<[u8; 65]> {
        let signature = self.ecdsa_pair.sign(message);
        Ok(signature.into())
    }
}
