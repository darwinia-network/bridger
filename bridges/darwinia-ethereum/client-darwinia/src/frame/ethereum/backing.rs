//! Darwinia Ethereum Backing
use core::marker::PhantomData;

use codec::{Decode, Encode};
use substrate_subxt::balances::Balances;
use substrate_subxt::system::System;
use substrate_subxt_proc_macro::{module, Call, Event, Store};
use support_ethereum::receipt::{EthereumReceiptProofThing, RedeemFor};

/// Ethereum Relay Pallet
#[module]
pub trait EthereumBacking: System + Balances {
    /// Ethereum transaction index
    type EthereumTransactionIndex: 'static + Encode + Decode + Send + Default + Clone + Sync;
}

//////
// Call
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

/// Submit SyncAuthoritiesChange call
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SyncAuthoritiesChange<T: EthereumBacking> {
    /// Runtime marker
    pub _runtime: PhantomData<T>,
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

/// Ethereum address
pub type EcdsaAddress = [u8; 20];

/// Someone lock some *RING*. [account, ecdsa address, asset type, amount]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct LockRing<T: EthereumBacking> {
    /// Account Id
    pub account_id: <T as System>::AccountId,
    /// Ecdsa address
    pub ecdsa_address: EcdsaAddress,
    /// Asset type
    pub asset_type: u8,
    /// amount
    pub amount: <T as Balances>::Balance,
}

/// Someone lock some *KTON*. [account, ecdsa address, asset type, amount]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct LockKton<T: EthereumBacking> {
    /// Account Id
    pub account_id: <T as System>::AccountId,
    /// Ecdsa address
    pub ecdsa_address: EcdsaAddress,
    /// Asset type
    pub asset_type: u8,
    /// amount
    pub amount: <T as Balances>::Balance,
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
