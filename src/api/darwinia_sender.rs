use crate::api::darwinia::AccountId;
use crate::error::{Error, Result};
use primitives::runtime::{DarwiniaRuntime, EcdsaMessage};
use primitives::{
	chain::RelayVotingState,
	frame::{
		bridge::relay_authorities::{
			AuthoritiesStoreExt, AuthoritiesToSignReturn, AuthoritiesToSignStoreExt,
			MmrRootsToSignStoreExt,
		},
		sudo::KeyStoreExt,
		technical_committee::MembersStoreExt,
	},
	runtime::EcdsaSignature,
};
use secp256k1::SecretKey;
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::sp_core::H256;
use substrate_subxt::{sp_core::Pair as PairTrait, Client, PairSigner};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::Web3;

#[derive(Debug, PartialEq)]
enum Role {
	Normal,
	TechnicalCommittee,
	Sudo,
	Authority,
}

fn build_roles(
	account: &AccountId,
	sudo: &AccountId,
	tech_comm_members: &[AccountId],
	authorities: &[AccountId],
) -> Vec<Role> {
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
	/// proxy real
	pub real: Option<AccountId>,

	/// ethereum url
	pub ethereum_url: String,
	/// authority signer raw ethereum seed
	pub ethereum_seed: Option<String>,
}

impl DarwiniaSender {
	/// Create a new Account
	pub fn new(
		seed: String,
		real: Option<String>,
		client: Client<DarwiniaRuntime>,
		ethereum_seed: Option<String>,
		ethereum_url: String,
	) -> DarwiniaSender {
		// signer to sign darwinia extrinsic
		let pair = Pair::from_string(&seed, None).unwrap(); // if not a valid seed
		let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
		let public = signer.signer().public().0;
		let account_id = AccountId::from(public);

		// real account, convert to account id
		let real = real.map(|real| AccountId::from(array_bytes::hex2array_unchecked!(real, 32)));

		DarwiniaSender {
			client,
			account_id,
			signer,
			real,
			ethereum_url,
			ethereum_seed,
		}
	}

	async fn roles(&self, block_number: Option<u32>) -> Result<Vec<Role>> {
		let block_number = block_number.map(|n| n.into());
		let block_hash: Option<H256> = self.client.block_hash(block_number).await?;

		let sudo = self.client.key(block_hash).await?;
		let tech_comm_members = self.client.members(block_hash).await?;
		let authorities = self
			.client
			.authorities(block_hash)
			.await?
			.iter()
			.map(|a| a.account_id.clone())
			.collect::<Vec<_>>();

		let roles = if let Some(real_account_id) = &self.real {
			build_roles(real_account_id, &sudo, &tech_comm_members, &authorities)
		} else {
			build_roles(&self.account_id, &sudo, &tech_comm_members, &authorities)
		};

		Ok(roles)
	}

	/// role names
	pub async fn role_names(&self, block_number: Option<u32>) -> Result<Vec<String>> {
		let roles = self
			.roles(block_number)
			.await?
			.iter()
			.map(|role| format!("{:?}", role))
			.collect::<Vec<String>>();
		Ok(roles)
	}

	/// is_sudo_key
	pub async fn is_sudo_key(&self, block_number: Option<u32>) -> Result<bool> {
		let roles = self.roles(block_number).await?;
		Ok(roles.contains(&Role::Sudo))
	}

	/// is_tech_comm_member
	pub async fn is_tech_comm_member(&self, block_number: Option<u32>) -> Result<bool> {
		let roles = self.roles(block_number).await?;
		Ok(roles.contains(&Role::TechnicalCommittee))
	}

	/// is_authority
	pub async fn is_authority(&self, block_number: Option<u32>) -> Result<bool> {
		let roles = self.roles(block_number).await?;
		Ok(roles.contains(&Role::Authority))
	}

	/// has_voted
	pub fn has_voted(&self, voting_state: RelayVotingState<AccountId>) -> bool {
		match &self.real {
			None => voting_state.contains(&self.account_id),
			Some(real) => voting_state.contains(real),
		}
	}

	/// sign
	pub fn ecdsa_sign(&self, message: &[u8]) -> Result<EcdsaSignature> {
		let web3 = Web3::new(Http::new(&self.ethereum_url)?);
		if let Some(ethereum_seed) = &self.ethereum_seed {
			let private_key = array_bytes::hex2bytes(&ethereum_seed[2..])
				.map_err(|_| Error::Hex2Bytes("ethereum_seed[2..]".into()))?;
			let secret_key = SecretKey::from_slice(&private_key)?;
			let signature = web3
				.accounts()
				.sign(message, SecretKeyRef::new(&secret_key))
				.signature;
			let mut buffer = [0u8; 65];
			buffer.copy_from_slice(signature.0.as_slice());
			Ok(EcdsaSignature(buffer))
		} else {
			Err(Error::NoAuthoritySignerSeed.into())
		}
	}

	/// need_to_sign_authorities
	pub async fn need_to_sign_authorities(
		&self,
		message: EcdsaMessage,
		block_number: Option<u32>,
	) -> Result<bool> {
		let block_number = block_number.map(|n| n.into());
		let block_hash: Option<H256> = self.client.block_hash(block_number).await?;

		let ret: AuthoritiesToSignReturn<DarwiniaRuntime> =
			self.client.authorities_to_sign(block_hash).await?;
		match ret {
			None => Ok(false),
			Some(r) => {
				if r.0 == message {
					let account_id = self.real.clone().unwrap_or_else(|| self.account_id.clone());
					let includes = r.1.iter().any(|a| a.0 == account_id);
					Ok(!includes)
				} else {
					Ok(false)
				}
			}
		}
	}

	/// need_to_mmr_root_of
	pub async fn need_to_sign_mmr_root_of(
		&self,
		block_number: u32,
		exec_block_number: Option<u32>,
	) -> Result<bool> {
		let exec_block_number = exec_block_number.map(|n| n.into());
		let exec_block_hash: Option<H256> = self.client.block_hash(exec_block_number).await?;

		let mmr_roots_to_sign = self
			.client
			.mmr_roots_to_sign(block_number, exec_block_hash)
			.await?;
		match mmr_roots_to_sign {
			None => Ok(false),
			Some(items) => {
				let account_id = self.real.clone().unwrap_or_else(|| self.account_id.clone());
				let includes = items.iter().any(|a| a.0 == account_id);
				Ok(!includes)
			}
		}
	}
}

#[test]
fn test_ecdsa() {
	let hash =
		array_bytes::hex2bytes("71e2f60faf6c7264cca14fb1a01260a787b4d18039cd8cd680aaff1e118c711d")
			.unwrap();
	let hash = hash.as_slice();
	// let hash = web3::signing::keccak256(message);
	let web3 = Web3::new(
		Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
	);
	let private_key =
		array_bytes::hex2bytes("8bd012fd2433d4fea852f437d6bb22d1e57dee7657cc1e703460ddeaae1a67ca")
			.unwrap();
	let secret_key = SecretKey::from_slice(&private_key).unwrap();
	let signature = web3
		.accounts()
		.sign(hash, SecretKeyRef::new(&secret_key))
		.signature;
	let mut buffer = [0u8; 65];
	buffer.copy_from_slice(signature.0.as_slice());
	println!("{:x?}", buffer);
}
