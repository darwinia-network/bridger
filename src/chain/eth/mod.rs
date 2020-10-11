//! Ethereum types
// mod confirmation;
mod ethash_proof;
mod header;
mod mmr_proof;
mod receipt;

pub use self::{
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthHeader, EthHeaderJson, EthHeaderRPC},
    mmr_proof::{HeaderStuffs, MMRProof, MMRProofJson},
    receipt::{EthereumReceiptProof, RedeemFor},
};
use codec::{Decode, Encode};

/// Ethereum HeaderThing
#[derive(Encode, Decode, Debug)]
pub struct HeaderThing {
    header: EthHeader,
    mmr_root: [u8; 32],
}

/// PendingHeader
pub type PendingHeader = (u64, u64, HeaderThing);
