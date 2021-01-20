//! Darwinia Bridge Relay Authorities
use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::sp_runtime::app_crypto::sp_core::H256;
use substrate_subxt::sp_core::bytes::to_hex;
use substrate_subxt::system::{System, SystemEventsDecoder};
use substrate_subxt_proc_macro::{module, Call, Event, Store};

/// AuthoritiesToSignReturn
pub type AuthoritiesToSignReturn<T> = Option<(
	<T as EthereumRelayAuthorities>::RelayAuthorityMessage,
	Vec<(
		<T as System>::AccountId,
		<T as EthereumRelayAuthorities>::RelayAuthoritySignature,
	)>,
)>;

/// AuthoritiesToSignReturn
pub type MmrRootsToSignReturn<T> = Option<
	Vec<(
		<T as System>::AccountId,
		<T as EthereumRelayAuthorities>::RelayAuthoritySignature,
	)>,
>;

/// Relay Authority
#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct RelayAuthority<AccountId, RelayAuthoritySigner, RingBalance, BlockNumber> {
	/// account_id
	pub account_id: AccountId,
	/// signer
	pub signer: RelayAuthoritySigner,
	/// Stake balance
	pub stake: RingBalance,
	/// BlockNumber
	pub term: BlockNumber,
}

/// Bridge Relay Authorities Pallet
#[module]
pub trait EthereumRelayAuthorities: System {
	/// Relay Authority
	type RelayAuthority: 'static + Encode + Decode + Send + Default;
	/// Relay authority signer
	type RelayAuthoritySigner: 'static + Encode + Decode + Send + Default;
	/// Relay signature
	type RelayAuthoritySignature: 'static + Encode + Decode + Send + Sync + Default;
	/// Relay signature
	type RelayAuthorityMessage: 'static + Encode + Decode + Send + Default;
}

//////
// Calls
//////

/// Submit authorities signature
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedAuthorities<T: EthereumRelayAuthorities> {
	/// signature
	pub signature: T::RelayAuthoritySignature,
}

/// Submit redeem call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitSignedMmrRoot<T: EthereumRelayAuthorities> {
	/// block_number
	pub block_number: <T as System>::BlockNumber,
	/// signature
	pub signature: T::RelayAuthoritySignature,
}

//////
// Events
//////

/// A New MMR Root Request to be Signed. [block number]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ScheduleMMRRoot<T: EthereumRelayAuthorities> {
	/// BlockNumber
	pub block_number: <T as System>::BlockNumber,
}

impl<T: EthereumRelayAuthorities> std::fmt::Display for ScheduleMMRRoot<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = format!(
			r#"
ScheduleMMRRoot {{
    block_number: {},
}}
"#,
			&self.block_number,
		);
		write!(f, "{}", msg)
	}
}

/// Authorities Signed. [term, new authorities, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AuthoritiesChangeSigned<T: EthereumRelayAuthorities> {
	/// term
	pub term: u32,
	/// new authorities
	pub new_authorities: Vec<T::RelayAuthoritySigner>,
	/// signatures
	pub signatures: Vec<(<T as System>::AccountId, T::RelayAuthoritySignature)>,
}

impl<T: EthereumRelayAuthorities> std::fmt::Display for AuthoritiesChangeSigned<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = format!(
			r#"
AuthoritiesChangeSigned {{
   term: {},
   new_authorities: {:?},
   signatures: {:?}
}}
"#,
			&self.term,
			&self
				.new_authorities
				.iter()
				.map(|n| to_hex(&n.encode(), false))
				.collect::<Vec<_>>(),
			&self
				.signatures
				.iter()
				.map(|s| { (to_hex(&s.0.encode(), false), to_hex(&s.1.encode(), false)) })
				.collect::<Vec<_>>()
		);
		write!(f, "{}", msg)
	}
}

/// MMR Root Signed. [block number, mmr root, message, signatures]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MMRRootSigned<T: EthereumRelayAuthorities> {
	/// block number
	pub block_number: u128,
	/// mmr root
	pub mmr_root: H256,
	/// The redeemed balance
	pub signatures: Vec<(<T as System>::AccountId, T::RelayAuthoritySignature)>,
}

/// ScheduleAuthoritiesChange. [message to sign]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ScheduleAuthoritiesChange<T: EthereumRelayAuthorities> {
	/// message
	pub message: T::RelayAuthorityMessage,
}

impl<T: EthereumRelayAuthorities> std::fmt::Display for ScheduleAuthoritiesChange<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = format!(
			r#"
ScheduleAuthoritiesChange {{
   message: {},
}}
"#,
			to_hex(&self.message.encode(), false)
		);
		write!(f, "{}", msg)
	}
}

//////
// Store
//////

/// Relay Authorities Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct Authorities<T: EthereumRelayAuthorities> {
	#[store(returns = Vec<T::RelayAuthority>)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// AuthoritiesToSign
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AuthoritiesToSign<T: EthereumRelayAuthorities> {
	#[store(returns = AuthoritiesToSignReturn<T>)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// AuthorityTerm
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct NextTerm<T: EthereumRelayAuthorities> {
	#[store(returns = u32)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// AuthorityTerm
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MMRRootsToSign<T: EthereumRelayAuthorities> {
	#[store(returns = MmrRootsToSignReturn<T>)]
	/// Block number
	pub block_number: <T as System>::BlockNumber,
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

/// AuthorityTerm
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct MMRRootsToSignKeys<T: EthereumRelayAuthorities> {
	#[store(returns = Vec<<T as System>::BlockNumber>)]
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::runtime::{DarwiniaRuntime, EcdsaMessage, EcdsaSignature};
	use substrate_subxt::sp_runtime::AccountId32;

	#[test]
	pub fn test_format_authorities_change_signed() {
		let a: AuthoritiesChangeSigned<DarwiniaRuntime> = AuthoritiesChangeSigned {
			term: 10,
			new_authorities: vec![[0u8; 20], [1; 20]],
			signatures: vec![(AccountId32::default(), EcdsaSignature::default())],
		};
		println!("{}", a);
	}

	#[test]
	pub fn test_format_schedule_mmr_root() {
		let a: ScheduleMMRRoot<DarwiniaRuntime> = ScheduleMMRRoot { block_number: 10 };

		println!("{}", a);
	}

	#[test]
	pub fn test_format_schedule_authorities_change() {
		let a: ScheduleAuthoritiesChange<DarwiniaRuntime> = ScheduleAuthoritiesChange {
			message: EcdsaMessage::default(),
		};

		println!("{}", a);
	}
}
