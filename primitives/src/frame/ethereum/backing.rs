//! Darwinia Ethereum Backing
use crate::chain::ethereum::{EthereumReceiptProofThing, RedeemFor};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use substrate_subxt::{
    balances::{Balances, BalancesEventsDecoder},
    system::{System, SystemEventsDecoder},
};
use substrate_subxt_proc_macro::{module, Call, Event, Store};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumBacking: System + Balances {
    /// Ethereum transaction index
    type EthereumTransactionIdex: 'static + Encode + Decode + Send + Default + Clone + Sync;
}

//////
/// Call
//////

/// Submit redeem call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct Redeem<T: EthereumBacking> {
    /// Runtime marker
    pub _runtime: PhantomData<T>,
    /// Token type
    pub act: RedeemFor,
    /// Ethereum Receipt Proof
    pub proof: EthereumReceiptProofThing,
}

//////
// Events
//////

/// Some one redeem some *RING*. [account, amount, transaction index]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RedeemRing<T: EthereumBacking> {
    /// Account Id
    pub account_id: <T as System>::AccountId,
    /// The redeemed balance
    pub balance: <T as Balances>::Balance,
    /// Transaction Id
    pub tx_id: u64,
}

/// Some one redeem some *KTON*. [account, amount, transaction index]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RedeemKton<T: EthereumBacking> {
    /// Account Id
    pub account_id: <T as System>::AccountId,
    /// The redeemed balance
    pub balance: <T as Balances>::Balance,
    /// Transaction Id
    pub tx_id: u64,
}

/// Some one redeem a deposit. [account, deposit id, amount, transaction index]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RedeemDeposit<T: EthereumBacking> {
    /// Account Id
    pub account_id: <T as System>::AccountId,
    /// The redeemed balance
    pub balance: <T as Balances>::Balance,
    /// Transaction Id
    pub tx_id: u64,
}

//////
/// Store
//////

/// PendingHeaders Storage
#[derive(Clone, Debug, Eq, PartialEq, Store, Decode, Encode)]
pub struct VerifiedProof<T: EthereumBacking> {
    #[store(returns = Option<bool>)]
    /// Receipt tx hash
    pub map: ([u8; 32], u64),
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}
