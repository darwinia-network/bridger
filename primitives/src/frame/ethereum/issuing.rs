//! Darwinia Ethereum Issuing
use crate::chain::ethereum::EthereumReceiptProofThing;
use crate::frame::ethereum::backing::EcdsaAddress;
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::{balances::Balances, system::System};
use substrate_subxt_proc_macro::{module, Call, Store};

/// Ethereum Issuing Pallet
#[module]
pub trait EthereumIssuing: System + Balances {
	/// Ethereum transaction index
	type EthereumTransactionIndex: 'static + Encode + Decode + Send + Default + Clone + Sync;
}

// Call

/// Submit register or lock call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterOrRedeemErc20<T: EthereumIssuing> {
	/// Runtime marker
	pub _runtime: PhantomData<T>,
    /// Backing address
    pub backing: EcdsaAddress,
	/// Ethereum Receipt Proof
	pub proof: EthereumReceiptProofThing,
}

/// verified proof Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Decode, Encode)]
pub struct VerifiedIssuingProof<T: EthereumIssuing> {
	#[store(returns = Option<bool>)]
	/// Receipt tx hash
	pub map: ([u8; 32], u64),
	/// Runtime marker
	pub _runtime: PhantomData<T>,
}
