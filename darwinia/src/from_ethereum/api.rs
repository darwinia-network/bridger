use crate::Darwinia;

use crate::error::{Error, Result};
use std::collections::HashMap;
type PendingRelayHeaderParcel<Relay> = <Relay as EthereumRelay>::PendingRelayHeaderParcel;
type AffirmationsReturn<Game> =
	HashMap<u64, HashMap<u32, Vec<<Game as EthereumRelayerGame>::RelayAffirmation>>>;

use primitives::frame::sudo::Sudo;
use primitives::frame::technical_committee::TechnicalCommittee;
use primitives::{
	chain::{
		ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
		RelayVotingState,
	},
	frame::{
		bridge::relay_authorities::EthereumRelayAuthorities,
		ethereum::{
			backing::{EthereumBacking, Redeem, RedeemCallExt},
			game::{AffirmationsStoreExt, EthereumRelayerGame},
			issuing::{
				EthereumIssuing, RedeemErc20, RedeemErc20CallExt, RegisterErc20,
				RegisterErc20CallExt,
			},
			relay::{
				Affirm, AffirmCallExt, ConfirmedBlockNumbersStoreExt, EthereumRelay,
				PendingRelayHeaderParcelsStoreExt, SetConfirmedParcel,
				VotePendingRelayHeaderParcel, VotePendingRelayHeaderParcelCallExt,
			},
		},
		proxy::{Proxy, ProxyCallExt},
		sudo::SudoCallExt,
		technical_committee::MembersStoreExt,
	},
};

use core::marker::PhantomData;

use super::Account;
use substrate_subxt::sp_runtime::traits::Verify;
use substrate_subxt::system::System;
use substrate_subxt::{Runtime, SignedExtension, SignedExtra};

/// Dawrinia API
#[derive(Clone)]
pub struct Ethereum2Darwinia<R: Runtime> {
	/// darwinia client
	pub darwinia: Darwinia<R>,
}

impl<R> Ethereum2Darwinia<R>
where
	R: Runtime
		+ Sudo
		+ TechnicalCommittee
		+ EthereumRelay
		+ Proxy
		+ EthereumRelayerGame
		+ EthereumBacking
		+ EthereumIssuing
		+ EthereumRelayAuthorities,
	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: std::marker::Send,
	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Sync,
	<R as Runtime>::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	<<R as substrate_subxt::Runtime>::Signature as Verify>::Signer:
		From<sp_keyring::sr25519::sr25519::Public>,
{
	pub fn new(darwinia: Darwinia<R>) -> Self {
		Self { darwinia }
	}

	/// Print Detail
	pub async fn account_detail(
		&self,
		block_number: Option<u32>,
		account: &Account<R>,
	) -> Result<()> {
		info!("🧔 ethereum => darwinia account");
		let mut roles = self.darwinia.account_role(&account.0).await?;
		if self.is_tech_comm_member(block_number, &account).await? {
			roles.push("TechnicalCommittee".to_string());
		}
		match &account.0.real {
			None => {
				info!("🧔 Relayer({:?}): 0x{:?}", roles, &account.0.account_id);
			}
			Some(real_account_id) => {
				info!("🧔 Proxy Relayer: 0x{:?}", &account.0.account_id);
				info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
			}
		}
		Ok(())
	}

	/// is_tech_comm_member
	pub async fn is_tech_comm_member(
		&self,
		block_number: Option<u32>,
		account: &Account<R>,
	) -> Result<bool> {
		let block_hash = self.darwinia.block_number2hash(block_number).await?;
		let tech_comm_members = self.darwinia.subxt.members(block_hash).await?;
		Ok(tech_comm_members.contains(account.0.real()))
	}

	/// set confirmed with sudo privilege
	pub async fn set_confirmed_parcel(
		&self,
		account: &Account<R>,
		parcel: EthereumRelayHeaderParcel,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
		let ex = self.darwinia.subxt.encode(SetConfirmedParcel {
			ethereum_relay_header_parcel: parcel,
			_runtime: PhantomData::default(),
		})?;
		Ok(self.darwinia.subxt.sudo(&account.0.signer, &ex).await?)
	}

	/// Vote pending relay header parcel
	pub async fn vote_pending_relay_header_parcel(
		&self,
		account: &Account<R>,
		pending: u64,
		aye: bool,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
		if self.is_tech_comm_member(None, &account).await? {
			match &account.0.real {
				Some(real) => {
					// proxy
					trace!("Proxy vote for {:?}", real);
					let vote = VotePendingRelayHeaderParcel {
						block_number: pending,
						aye,
						_runtime: PhantomData::default(),
					};

					let ex = self.darwinia.subxt.encode(vote).unwrap();
					let ex_hash = self
						.darwinia
						.subxt
						.proxy(&account.0.signer, real.clone(), Some(proxy_type), &ex)
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
	pub async fn affirmations(&self) -> Result<AffirmationsReturn<R>> {
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
				result.insert(game_id, HashMap::<u32, Vec<R::RelayAffirmation>>::new());
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

	// /// affirmations contains block?
	// pub fn contains(affirmations: &[R::RelayAffirmation], block: u64) -> bool {
	// 	for affirmation in affirmations {
	// 		let blocks: &Vec<u64> = &affirmation
	// 			.relay_header_parcels
	// 			.iter()
	// 			.map(|bp| bp.header.number)
	// 			.collect();
	// 		if blocks.contains(&block) {
	// 			return true;
	// 		}
	// 	}
	//
	// 	// TODO: Checking the equality of the affirmations
	//
	// 	// TODO: If there is an affirmation with larger block number, then agree and join in the game.
	//
	// 	// TODO: How to play and join the game
	// 	false
	// }

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
	pub async fn pending_headers(&self) -> Result<Vec<PendingRelayHeaderParcel<R>>> {
		Ok(self
			.darwinia
			.subxt
			.pending_relay_header_parcels(None)
			.await?)
	}

	/// Submit affirmation
	pub async fn affirm(
		&self,
		account: &Account<R>,
		parcel: EthereumRelayHeaderParcel,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
		match &account.0.real {
			Some(real) => {
				trace!("Proxy call `affirm` for {:?}", real);
				let affirm = Affirm {
					_runtime: PhantomData::default(),
					ethereum_relay_header_parcel: parcel,
					ethereum_relay_proofs: None,
				};

				let ex = self.darwinia.subxt.encode(affirm).unwrap();
				Ok(self
					.darwinia
					.subxt
					.proxy(&account.0.signer, real.clone(), Some(proxy_type), &ex)
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
		account: &Account<R>,
		redeem_for: RedeemFor,
		proof: EthereumReceiptProofThing,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
		let ethereum_tx_hash = proof
			.header
			.hash
			.map(|hash| array_bytes::bytes2hex("", &hash))
			.ok_or(Error::NoHeaderHashInEthereumReceiptProofOfThing)?;
		match &account.0.real {
			Some(real) => {
				trace!(
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
					.proxy(&account.0.signer, real.clone(), Some(proxy_type), &ex)
					.await?)
			}
			None => {
				trace!(
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
	pub fn has_voted(
		&self,
		account: &Account<R>,
		voting_state: RelayVotingState<<R as System>::AccountId>,
	) -> bool
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
		match &account.0.real {
			None => voting_state.contains(&account.0.account_id),
			Some(real) => voting_state.contains(real),
		}
	}

	/// register erc20
	pub async fn register_erc20(
		&self,
		account: &Account<R>,
		proof: EthereumReceiptProofThing,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
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
					.proxy(&account.0.signer, real.clone(), Some(proxy_type), &ex)
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
		account: &Account<R>,
		proof: EthereumReceiptProofThing,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where
		<R as System>::Address: From<<R as System>::AccountId>,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	{
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
					.proxy(&account.0.signer, real.clone(), Some(proxy_type), &ex)
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
