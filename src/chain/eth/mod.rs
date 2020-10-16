//! Ethereum types
// mod confirmation;
mod ethash_proof;
mod header;
mod header_thing;
mod mmr_proof;
mod receipt;
mod runtime;

pub use self::{
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthereumHeader, EthereumHeaderJson, EthereumHeaderRPC},
    header_thing::{
        EthereumHeaderThing, EthereumHeaderThingJson, EthereumHeaderThingWithConfirmationJson,
    },
    mmr_proof::{HeaderStuff, HeaderStuffJson, MMRProof, MMRProofJson},
    receipt::{
        EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};
