//! Ethereum types
// mod confirmation;
mod ethash_proof;
mod header;
mod mmr_proof;

pub use self::{
    ethash_proof::{EthashProof, EthashProofJson},
    header::{EthHeader, EthHeaderJson, EthHeaderRPC},
    mmr_proof::{HeaderStuffs, MMRProof, MMRProofJson},
};
