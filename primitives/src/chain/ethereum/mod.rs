//! Ethereum types
// mod confirmation;
mod ethash;
mod header;
mod mmr;
mod parcel;
mod proof;
mod receipt;

pub use self::{
    ethash::{EthashProof, EthashProofJson},
    header::{EthereumHeader, EthereumHeaderJson, EthereumHeaderRPC},
    mmr::{MMRProof, MMRProofJson},
    parcel::{EthereumRelayHeaderParcel, EthereumRelayHeaderParcelJson},
    proof::{EthereumRelayProofs, EthereumRelayProofsJson},
    receipt::{
        EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};
