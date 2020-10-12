//! Ethereum types
// mod confirmation;
mod ethash_proof;
mod header;
mod header_thing;
mod mmr_proof;
mod receipt;

pub use self::{
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthereumHeader, EthereumHeaderJson, EthereumHeaderRPC},
    header_thing::{HeaderThing, HeaderThingJson, HeaderThingWithConfirmationJson},
    mmr_proof::{HeaderStuff, HeaderStuffJson, MMRProof, MMRProofJson},
    receipt::{
        EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};

/// PendingHeader
pub type PendingHeader = (u64, u64, HeaderThing);
