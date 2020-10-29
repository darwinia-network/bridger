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
    mmr::{MMRProof, MMRProofJson, MMRRoot, MMRRootJson},
    parcel::{EthereumRelayHeaderParcel, EthereumRelayHeaderParcelJson},
    proof::{EthereumRelayProofs, EthereumRelayProofsJson},
    receipt::{
        EthereumReceipt, EthereumReceiptProof, EthereumReceiptProofJson, EthereumReceiptProofThing,
        EthereumReceiptProofThingJson, RedeemFor,
    },
};
