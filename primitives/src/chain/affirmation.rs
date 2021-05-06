use codec::{Decode, Encode};
use crate::chain::ethereum::EthereumRelayHeaderParcel;
use substrate_subxt::Runtime;
use std::fmt::Display;

/// Game id, round and the index under the round point to a unique affirmation AKA affirmation id
#[derive(Clone, PartialEq, Encode, Decode, Default, Debug)]
pub struct RelayAffirmationId<RelayHeaderId> {
	/// Relay header id aka game id
	pub relay_header_id: RelayHeaderId,
	/// Round index
	pub round: u32,
	/// Index of a affirmation list which under a round
	pub index: u32,
}

/// Relay Affirmations
#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct RelayAffirmation<RelayHeaderParcel, Relayer, Balance, RelayHeaderId> {
	/// Relayer
	pub relayer: Relayer,
	/// Relay header parcels
	pub relay_header_parcels: Vec<RelayHeaderParcel>,
	/// Stake balance
	pub stake: Balance,
	/// Affirmation ID
	pub maybe_extended_relay_affirmation_id: Option<RelayAffirmationId<RelayHeaderId>>,
	/// Verified
	pub verified_on_chain: bool,
}

impl<AccountId: Display, Balance: Display> std::fmt::Display
for RelayAffirmation<
	EthereumRelayHeaderParcel,
	AccountId,
	Balance,
	RelayAffirmationId<u64>,
>
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let msg = format!(
			"{{\n  relayer: {}\n  balance: {}\n  relayer_header: [{}\n  ]\n  id: {:?}\n  verified: {}\n}}",
			self.relayer,
			self.stake,
			self.relay_header_parcels.iter().fold(String::from(""), |acc, relay| {
				if acc.is_empty() {
					format!("\n  {{\n{}\n  }}", relay)
				} else {
					format!("{}, \n  {{{}\n  }}", acc, relay)
				}
			}),
			self.maybe_extended_relay_affirmation_id,
			self.verified_on_chain
		);
		write!(f, "{}", msg)
	}
}

/// Info for keeping track of a proposal being voted on.
#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct RelayVotingState<TechnicalMember> {
	/// The current set of technical members that approved it.
	pub ayes: Vec<TechnicalMember>,
	/// The current set of technical members that rejected it.
	pub nays: Vec<TechnicalMember>,
}

impl<TechnicalMember: PartialEq> RelayVotingState<TechnicalMember> {
	/// contains the vote of the account
	pub fn contains(&self, account: &TechnicalMember) -> bool {
		self.ayes.contains(account) || self.nays.contains(account)
	}
}
