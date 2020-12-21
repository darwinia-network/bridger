//! Ethereum types
// mod confirmation;
mod ethash;
mod block;
mod mmr;
mod parcel;
mod proof;
mod receipt;

pub use self::{
    ethash::{EthashProof, EthashProofJson},
    block::{EthereumHeader, EthereumHeaderJson, EthereumBlockRPC},
    mmr::{MMRProof, MMRProofJson, MMRRoot, MMRRootJson},
    parcel::{EthereumRelayHeaderParcel, EthereumRelayHeaderParcelJson},
    proof::{EthereumRelayProofs, EthereumRelayProofsJson},
    receipt::{
        EthereumReceipt, EthReceiptBody, EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};
